use std::{
    cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd},
    collections::BTreeSet,
    mem::replace,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn left(self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn right(self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Track {
    NorthSouth,
    EastWest,
    SouthWestNorthEast,
    NorthWestSouthEast,
    Intersection,
}

impl Track {
    fn from_character(character: char) -> Option<Track> {
        match character {
            '|' | '^' | 'v' => Some(Track::NorthSouth),
            '-' | '<' | '>' => Some(Track::EastWest),
            '/' => Some(Track::SouthWestNorthEast),
            '\\' => Some(Track::NorthWestSouthEast),
            '+' => Some(Track::Intersection),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Turn {
    Left,
    Straight,
    Right,
}

impl Turn {
    fn next(self) -> Turn {
        match self {
            Turn::Left => Turn::Straight,
            Turn::Straight => Turn::Right,
            Turn::Right => Turn::Left,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Cart {
    x: usize,
    y: usize,
    heading: Direction,
    next_turn: Turn,
}

// Sort order of carts is top-to-bottom, left-to-right
// (so that sorting puts them in the order in which they should move).
// Carts are considered equal if they are in the same place.

impl PartialEq for Cart {
    fn eq(&self, other: &Cart) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Cart {}

impl Ord for Cart {
    fn cmp(&self, other: &Cart) -> Ordering {
        match self.y.cmp(&other.y) {
            Ordering::Equal => self.x.cmp(&other.x),
            unequal => unequal,
        }
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Cart) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Cart {
    pub fn new(x: usize, y: usize, heading: Direction) -> Cart {
        Cart {
            x,
            y,
            heading,
            next_turn: Turn::Left,
        }
    }

    pub fn step(&self, track: &[Vec<Option<Track>>]) -> Cart {
        let (x, y) = match self.heading {
            Direction::North => (self.x, self.y - 1),
            Direction::South => (self.x, self.y + 1),
            Direction::East => (self.x + 1, self.y),
            Direction::West => (self.x - 1, self.y),
        };

        let landed_on = track[y][x].expect("Cart moved off the track.");

        let new_heading = match landed_on {
            Track::NorthSouth | Track::EastWest => self.heading,

            Track::Intersection => match self.next_turn {
                Turn::Left => self.heading.left(),
                Turn::Straight => self.heading,
                Turn::Right => self.heading.right(),
            },

            Track::NorthWestSouthEast => match self.heading {
                Direction::North => Direction::West,
                Direction::South => Direction::East,
                Direction::East => Direction::South,
                Direction::West => Direction::North,
            },

            Track::SouthWestNorthEast => match self.heading {
                Direction::North => Direction::East,
                Direction::South => Direction::West,
                Direction::East => Direction::North,
                Direction::West => Direction::South,
            },
        };

        let new_next_turn = match landed_on {
            Track::Intersection => self.next_turn.next(),
            _ => self.next_turn,
        };

        Cart {
            x,
            y,
            heading: new_heading,
            next_turn: new_next_turn,
        }
    }

    pub fn location(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

pub fn parse_input(input: &str) -> (Vec<Vec<Option<Track>>>, BTreeSet<Cart>) {
    let mut track = Vec::new();
    let mut carts = BTreeSet::new();

    for (row, line) in input.lines().enumerate() {
        let mut track_row = Vec::with_capacity(line.len());

        for (column, character) in line.chars().enumerate() {
            track_row.push(Track::from_character(character));

            match character {
                '^' => {
                    carts.insert(Cart::new(column, row, Direction::North));
                }
                'v' => {
                    carts.insert(Cart::new(column, row, Direction::South));
                }
                '>' => {
                    carts.insert(Cart::new(column, row, Direction::East));
                }
                '<' => {
                    carts.insert(Cart::new(column, row, Direction::West));
                }
                _ => {}
            }
        }

        track.push(track_row);
    }

    (track, carts)
}

pub fn simulate_until_collision(
    track: &[Vec<Option<Track>>],
    mut carts: BTreeSet<Cart>,
) -> (usize, usize) {
    loop {
        let mut carts_already_moved = BTreeSet::new();
        let mut carts_not_yet_moved = carts.clone();

        for cart in carts.iter() {
            carts_not_yet_moved.remove(&cart);
            let moved_cart = cart.step(track);
            let location = moved_cart.location();

            if carts_not_yet_moved.contains(&moved_cart) || !carts_already_moved.insert(moved_cart)
            {
                // We have a collision
                return location;
            }
        }

        replace(&mut carts, carts_already_moved);
    }
}

pub fn simulate_until_only_one_cart_remains(
    track: &[Vec<Option<Track>>],
    mut carts: BTreeSet<Cart>,
) -> (usize, usize) {
    while carts.len() > 1 {
        let mut carts_already_moved = BTreeSet::new();
        let mut carts_not_yet_moved = carts.clone();
        let mut carts_destroyed = BTreeSet::new();

        for cart in carts.iter() {
            if carts_destroyed.contains(cart) {
                continue;
            }

            carts_not_yet_moved.remove(&cart);
            let moved_cart = cart.step(track);

            if carts_not_yet_moved.contains(&moved_cart) {
                carts_not_yet_moved.remove(&moved_cart);
                carts_destroyed.insert(moved_cart);
                continue;
            }

            if carts_already_moved.contains(&moved_cart) {
                carts_already_moved.remove(&moved_cart);
            } else {
                carts_already_moved.insert(moved_cart);
            }
        }

        replace(&mut carts, carts_already_moved);
    }

    let last_cart = carts.iter().last().unwrap();
    last_cart.location()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_straight_track() {
        let input = include_str!("example_straight_track.txt");
        let (track, carts) = parse_input(&input);
        let collision = simulate_until_collision(&track, carts);
        assert_eq!((0, 3), collision);
    }

    #[test]
    fn part1_example() {
        let input = include_str!("example_track.txt");
        let (track, carts) = parse_input(&input);
        let collision = simulate_until_collision(&track, carts);
        assert_eq!((7, 3), collision);
    }

    #[test]
    fn part2_example() {
        let input = include_str!("part2-example.txt");
        let (track, carts) = parse_input(&input);
        let collision = simulate_until_only_one_cart_remains(&track, carts);
        assert_eq!((6, 4), collision);
    }
}
