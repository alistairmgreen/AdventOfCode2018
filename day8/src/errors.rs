use std::{error::Error, fmt, num::ParseIntError};

#[derive(Debug, Clone, Copy)]
pub enum TreeParseError {
    InvalidNumber,
    MissingData,
}

impl fmt::Display for TreeParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TreeParseError::InvalidNumber => write!(f, "Cannot parse number"),
            TreeParseError::MissingData => write!(f, "Unexpected end of data"),
        }
    }
}

impl Error for TreeParseError {}

impl From<ParseIntError> for TreeParseError {
    fn from(_: ParseIntError) -> TreeParseError {
        TreeParseError::InvalidNumber
    }
}