fn main() {
    let input = include_str!("puzzle_input.txt");
    let reacted = react(&input);
    let remaining_units = reacted.len();

    println!("The reaction has cut the polymer from {} units to {} units.", input.len(), remaining_units);
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
}
