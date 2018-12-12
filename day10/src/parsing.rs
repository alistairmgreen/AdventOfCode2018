use crate::vectors::Vector2D;
use lazy_static::*;
use regex::Regex;
use std::{error, fmt, num::ParseIntError};

lazy_static! {
    static ref INPUT_REGEX: Regex =
        Regex::new(r"position=<\s*(\-?\d+),\s*(\-?\d+)>\s+velocity=<\s*(\-?\d+),\s*(\-?\d+)>")
            .expect("Cannot parse regular expression");
}

#[derive(Debug, Copy, Clone)]
pub enum ParseError {
    IncorrectFormat,
    InvalidNumber,
}

impl From<ParseIntError> for ParseError {
    fn from(_: ParseIntError) -> ParseError {
        ParseError::InvalidNumber
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::IncorrectFormat => write!(f, "Cannot parse input"),
            ParseError::InvalidNumber => write!(f, "Invalid number"),
        }
    }
}

impl error::Error for ParseError {}

pub fn parse_input(input: &str) -> Result<(Vector2D, Vector2D), ParseError> {
    match INPUT_REGEX.captures(input) {
        Some(ref captures) if captures.len() == 5 => {
            let position = Vector2D {
                x: captures[1].parse()?,
                y: captures[2].parse()?,
            };

            let velocity = Vector2D {
                x: captures[3].parse()?,
                y: captures[4].parse()?,
            };

            Ok((position, velocity))
        }
        _ => Err(ParseError::IncorrectFormat),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let (position, velocity) =
            parse_input("position=< 21518, -21209> velocity=<-2,  2>").unwrap();
        assert_eq!(
            Vector2D {
                x: 21518,
                y: -21209
            },
            position
        );
        assert_eq!(Vector2D { x: -2, y: 2 }, velocity);
    }
}
