use std::collections::HashMap;

fn main() {
    let input = include_str!("puzzle_input.txt");
    let reacted = react(&input);
    let remaining_units = reacted.len();

    println!(
        "The reaction has cut the polymer from {} units to {} units.",
        input.len(),
        remaining_units
    );

    let (removed_letter, optimized) = optimize(&input);

    println!("Removing {} allows the polymer to react down to {} units.", removed_letter, optimized.len());
}

fn units_react(left: char, right: char) -> bool {
    left.is_uppercase() != right.is_uppercase()
        && left.to_ascii_lowercase() == right.to_ascii_lowercase()
}

fn react(polymer: &str) -> String {
    let mut reacted = String::new();

    for original_character in polymer.chars() {
        match reacted.chars().last() {
            Some(reacted_character) => {
                if units_react(reacted_character, original_character) {
                    reacted.pop();
                } else {
                    reacted.push(original_character);
                }
            }

            None => {
                reacted.push(original_character);
            }
        }
    }

    reacted
}

fn optimize(polymer: &str) -> (char, String) {
    let mut solutions = HashMap::with_capacity(26);

    for letter in alphabet() {
        let mut filtered = polymer.to_string();
        filtered.retain(|c| c != letter && c != letter.to_ascii_uppercase());
        let optimized = react(&filtered);
        solutions.insert(letter, optimized);
    }

    let (&removed_letter, optimized) = solutions
        .iter()
        .min_by_key(|(_, sequence)| sequence.len())
        .unwrap();

    (removed_letter, optimized.to_owned())
}

fn alphabet() -> Alphabet {
    Alphabet { offset: 0 }
}

struct Alphabet {
    offset: u8,
}

impl Iterator for Alphabet {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        if self.offset < 26 {
            let letter = (b'a' + self.offset) as char;
            self.offset += 1;

            Some(letter)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_reactions() {
        assert!(units_react('a', 'A'));
        assert!(units_react('A', 'a'));
        assert!(units_react('z', 'Z'));

        assert!(!units_react('a', 'a'));
        assert!(!units_react('A', 'A'));
        assert!(!units_react('a', 'B'));
    }

    #[test]
    fn example_reaction() {
        let reacted = react("dabAcCaCBAcCcaDA");
        assert_eq!(String::from("dabCBAcaDA"), reacted);
    }

    #[test]
    fn example_optimization() {
        let (removed, optimized) = optimize("dabAcCaCBAcCcaDA");
        assert_eq!('c', removed);
        assert_eq!("daDA".to_string(), optimized);
    }
}
