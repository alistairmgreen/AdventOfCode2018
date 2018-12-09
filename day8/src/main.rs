use std::num::ParseIntError;
use day8::{Node, errors::TreeParseError};

fn main() -> Result<(), TreeParseError> {
    let input = include_str!("puzzle_input.txt")
        .split_whitespace()
        .map(|n| n.parse())
        .collect::<Result<Vec<i32>, ParseIntError>>()?;
    
    let mut iterator = input.iter();
    let tree = Node::read(&mut iterator)?;

    println!("The sum of all metadata is {}", tree.sum_metadata());

    Ok(())
}

