use std::ops::{Index, IndexMut};

fn main() {
    let grid = compute_power_grid(300, 5535);
    let (x, y) = find_best_square(&grid);
    println!("The 3x3 square with the highest power is {},{}", x, y);
}

fn hundreds(x: i32) -> i32 {
    ((x % 1000) - (x % 100)) / 100
}

fn power(x: i32, y: i32, serial: i32) -> i32 {
    let rack = x + 10;
    hundreds(rack * (rack * y + serial)) - 5
}

pub struct Grid<T> {
    rank: usize,
    items: Vec<T>,
}

impl<T: Default + Copy> Grid<T> {
    pub fn new(rank: usize) -> Grid<T> {
        Grid {
            rank,
            items: vec![T::default(); rank * rank],
        }
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = [T];
    fn index(&self, row: usize) -> &[T] {
        let start = row * self.rank;
        &self.items[start..start + self.rank]
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, row: usize) -> &mut [T] {
        let start = row * self.rank;
        &mut self.items[start..start + self.rank]
    }
}

fn compute_power_grid(dimension: usize, serial: i32) -> Grid<i32> {
    let mut grid = Grid::new(dimension + 1);

    #[allow(clippy::needless_range_loop)]
    for x in 1..=dimension {
        for y in 1..dimension {
            grid[x][y] = power(x as i32, y as i32, serial);
        }
    }

    grid
}

fn find_best_square(grid: &Grid<i32>) -> (usize, usize) {
    let dimension = grid.rank - 1;
    let last = dimension - 3;
    let mut best_coordinate = (0, 0);
    let mut best_power = 0;

    for x in 1..=last {
        for y in 1..=last {
            let power = total_power_3x3(x, y, &grid);
            if power > best_power {
                best_power = power;
                best_coordinate = (x, y);
            }
        }
    }

    best_coordinate
}

fn total_power_3x3(x: usize, y: usize, grid: &Grid<i32>) -> i32 {
    let mut power = 0;
    for x_offset in 0..=2 {
        let column = &grid[x + x_offset];
        for y_offset in 0..=2 {
            power += column[y + y_offset];
        }
    }

    power
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hundreds() {
        assert_eq!(2, hundreds(1234));
        assert_eq!(0, hundreds(99));
    }

    #[test]
    fn example_power() {
        assert_eq!(4, power(3, 5, 8));
        assert_eq!(-5, power(122, 79, 57));
        assert_eq!(0, power(217, 196, 39));
        assert_eq!(4, power(101, 153, 71));
    }

    #[test]
    fn example_grid() {
        let grid = compute_power_grid(300, 18);
        assert_eq!(4, grid[33][45]);
        assert_eq!(-2, grid[32][44]);
    }

    #[test]
    fn part1_example1() {
        let grid = compute_power_grid(300, 18);
        assert_eq!((33, 45), find_best_square(&grid));
    }

    #[test]
    fn part1_example2() {
        let grid = compute_power_grid(300, 42);
        assert_eq!((21, 61), find_best_square(&grid));
    }
}
