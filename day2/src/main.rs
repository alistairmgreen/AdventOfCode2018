use std::{collections::HashMap, fs::read_to_string, io};

fn main() -> Result<(), io::Error> {
    let input = read_to_string("puzzle_input.txt")?;
    let ids: Vec<&str> = input.lines().collect();

    // Part 1
    let checksum = calculate_checksum(&ids);
    println!("The checksum is {}.", checksum);

    // Part 2
    match find_box_ids(&ids) {
        Some((a, b)) => {
            println!(
                "The boxes that differ by only one character are:\n{}\n{}\n",
                a, b
            );

            let common = common_letters(a, b);
            println!("The common letters are: {}", common);
        }
        None => {
            println!("Could not find a pair of boxes that differ by one character.");
        }
    }

    Ok(())
}

fn calculate_checksum(ids: &[&str]) -> usize {
    let mut two_letters_same = 0;
    let mut three_letters_same = 0;

    for id in ids {
        let letters = letter_frequency(id);

        if letters.values().any(|count| *count == 2) {
            two_letters_same += 1;
        }

        if letters.values().any(|count| *count == 3) {
            three_letters_same += 1;
        }
    }

    two_letters_same * three_letters_same
}

fn letter_frequency(text: &str) -> HashMap<char, usize> {
    let mut frequency = HashMap::new();

    for letter in text.chars() {
        let occurences = frequency.entry(letter).or_insert(0);
        *occurences += 1;
    }

    frequency
}

fn count_differences(a: &str, b: &str) -> usize {
    a.chars().zip(b.chars()).filter(|&(a, b)| a != b).count()
}

/// Finds the pair of box IDs that differ by exactly one character.
fn find_box_ids<'a>(all_ids: &[&'a str]) -> Option<(&'a str, &'a str)> {
    for (index, &first_id) in all_ids[0..all_ids.len() - 1].iter().enumerate() {
        if let Some(second_id) = all_ids[index + 1..]
            .iter()
            .find(|&&id| count_differences(first_id, id) == 1)
        {
            return Some((first_id, second_id));
        }
    }

    None
}

fn common_letters(a: &str, b: &str) -> String {
    a.chars()
        .zip(b.chars())
        .filter_map(|(a, b)| if a == b { Some(a) } else { None })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn letter_frequency_returns_correct_values() {
        let frequencies = letter_frequency("bababc");

        assert_eq!(2, frequencies[&'a']);
        assert_eq!(3, frequencies[&'b']);
        assert_eq!(1, frequencies[&'c']);
    }

    #[test]
    fn checksum_part1_example() {
        let ids = vec![
            "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
        ];

        let checksum = calculate_checksum(&ids);
        assert_eq!(12, checksum);
    }

    #[test]
    fn count_character_differences_example1() {
        assert_eq!(2, count_differences("abcde", "axcye"));
    }

    #[test]
    fn count_character_differences_example2() {
        assert_eq!(1, count_differences("fghij", "fguij"));
    }

    #[test]
    fn find_correct_pair_of_box_ids() {
        let ids = vec![
            "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
        ];

        assert_eq!(Some(("fghij", "fguij")), find_box_ids(&ids));
    }

    #[test]
    fn common_letters_example() {
        assert_eq!("fgij", common_letters("fghij", "fguij"));
    }
}
