mod collections;
use crate::collections::CircularList;

fn main() {
    let score = high_score(476, 71_431);
    println!("The winning score with 71,431 turns is {}", score);

    let score = high_score(476, 7_143_100);
    println!("The winning score with 7,143,100 turns is {}", score);
}

#[derive(Debug)]
struct MarbleGame {
    marbles: CircularList,
    current_marble: usize,
}

impl MarbleGame {
    pub fn with_capacity(marble_count: usize) -> MarbleGame {
        MarbleGame {
            marbles: CircularList::with_capacity(marble_count + 1),
            current_marble: 0,
        }
    }

    /// Places a marble in the circle and returns the resulting score.
    pub fn place_marble(&mut self, number: usize) -> usize {
        if number % 23 == 0 {
            let mut removed_marble = self
                .marbles
                .left_of(self.current_marble)
                .expect("Marble 1 place to left does not exist");
            for n in 2..=7 {
                removed_marble = self
                    .marbles
                    .left_of(removed_marble)
                    .unwrap_or_else(|| panic!("Marble {} places to left does not exist", n));
            }

            self.current_marble = self
                .marbles
                .right_of(removed_marble)
                .expect("Next current marble does not exist");
            self.marbles.remove(removed_marble);

            number + removed_marble
        } else {
            let marble_to_right = self
                .marbles
                .right_of(self.current_marble)
                .expect("Marble to right of current does not exist");
            self.marbles.insert_after(marble_to_right, number);
            self.current_marble = number;

            0
        }
    }
}

fn high_score(players: usize, last_marble: usize) -> usize {
    let mut scores = vec![0; players];
    let mut game = MarbleGame::with_capacity(last_marble);
    let mut player: usize = 0;

    for n in 1..=last_marble {
        scores[player] += game.place_marble(n);
        player = (player + 1) % players;
    }

    scores.iter().cloned().max().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nine_player_high_score() {
        assert_eq!(32, high_score(9, 25));
    }

    #[test]
    fn ten_player_high_score() {
        assert_eq!(8317, high_score(10, 1618));
    }

    #[test]
    fn thirteen_player_high_score() {
        assert_eq!(146_373, high_score(13, 7999));
    }

    #[test]
    fn seventeen_player_high_score() {
        assert_eq!(2764, high_score(17, 1104));
    }

    #[test]
    fn twenty_one_player_high_score() {
        assert_eq!(54_718, high_score(21, 6111));
    }

    #[test]
    fn thirty_player_high_score() {
        assert_eq!(37_305, high_score(30, 5807));
    }
}
