use std::{collections::HashMap, fs::read_to_string, io};

fn main() -> Result<(), io::Error> {
    let input = read_to_string("puzzle_input.txt")?;

    let mut two_letters_same = 0;
    let mut three_letters_same = 0;

    for line in input.lines() {
        let letters = letter_frequency(line);

        if letters.values().any(|count| *count == 2) {
            two_letters_same += 1;
        }

        if letters.values().any(|count| *count == 3) {
            three_letters_same += 1;
        }
    }

    println!(
        "{} entries have a letter that occurs twice.",
        two_letters_same
    );
    println!(
        "{} entries have a letter that occurs three times.",
        three_letters_same
    );
    let checksum = two_letters_same * three_letters_same;

    println!("The checksum is {}.", checksum);

    Ok(())
}

fn letter_frequency(text: &str) -> HashMap<char, usize> {
    let mut frequency = HashMap::new();

    for letter in text.chars() {
        let occurences = frequency.entry(letter).or_insert(0);
        *occurences += 1;
    }

    frequency
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
}
