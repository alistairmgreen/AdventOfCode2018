fn main() {
    let score = high_score(476, 71_431);
    println!("The winning score is {}", score);
}

#[derive(Debug)]
struct MarbleGame {
    marbles: Vec<usize>,
    current_marble: usize,
}

impl MarbleGame {
    pub fn new() -> MarbleGame {
        MarbleGame {
            marbles: vec![0],
            current_marble: 0,
        }
    }

    /// Places a marble in the circle and returns the resulting score.
    pub fn place_marble(&mut self, number: usize) -> usize {
        if number % 23 == 0 {
            let index_to_remove = if self.current_marble >= 7 {
                self.current_marble - 7
            } else {
                (self.current_marble as i32 - 7 + self.marbles.len() as i32) as usize
            };
            let removed_marble = self.marbles[index_to_remove];
            self.marbles.remove(index_to_remove);
            self.current_marble = index_to_remove % self.marbles.len();

            number + removed_marble
        } else {
            let next_index = (self.current_marble + 2) % self.marbles.len();
            self.marbles.insert(next_index, number);
            self.current_marble = next_index;

            0
        }
    }
}

fn high_score(players: usize, last_marble: usize) -> usize {
    let mut scores = vec![0; players];
    let mut game = MarbleGame::new();
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
    fn place_23rd_marble() {
        let mut game = MarbleGame {
            current_marble: 13,
            marbles: vec![
                0, 16, 8, 17, 4, 18, 9, 19, 2, 20, 10, 21, 5, 22, 11, 1, 12, 6, 13, 3, 14, 7, 15,
            ],
        };

        assert_eq!(22, game.marbles[game.current_marble]);

        let score = game.place_marble(23);
        assert_eq!(32, score);
        assert_eq!(
            vec![0, 16, 8, 17, 4, 18, 19, 2, 20, 10, 21, 5, 22, 11, 1, 12, 6, 13, 3, 14, 7, 15],
            game.marbles
        );
        assert_eq!(19, game.marbles[game.current_marble]);
    }

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
