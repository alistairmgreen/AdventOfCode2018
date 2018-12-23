use lazy_static::lazy_static;
use regex::Regex;
use std::{fmt, num::ParseIntError, str::FromStr};

lazy_static! {
    static ref INPUT_REGEX: Regex =
        Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>,\s+r=(-?\d+)").unwrap();
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Error {
    IncorrectFormat,
    InvalidNumber,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::IncorrectFormat => write!(f, "Incorrect format"),
            Error::InvalidNumber => write!(f, "Cannot parse number"),
        }
    }
}

impl std::error::Error for Error {}

impl From<ParseIntError> for Error {
    fn from(_: ParseIntError) -> Error {
        Error::InvalidNumber
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Nanobot {
    pub x: i64,
    pub y: i64,
    pub z: i64,
    pub radius: i64,
}

impl Nanobot {
    pub fn manhattan_distance(&self, other: &Nanobot) -> i64 {
        i64::abs(self.x - other.x) + i64::abs(self.y - other.y) + i64::abs(self.z - other.z)
    }
}

impl FromStr for Nanobot {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match INPUT_REGEX.captures(s) {
            Some(ref captures) if captures.len() == 5 => {
                let x = captures[1].parse()?;
                let y = captures[2].parse()?;
                let z = captures[3].parse()?;
                let radius = captures[4].parse()?;

                Ok(Nanobot { x, y, z, radius })
            }

            _ => Err(Error::IncorrectFormat),
        }
    }
}
