#[macro_use]
extern crate failure;

#[macro_use]
extern crate lazy_static;

use regex::Regex;

lazy_static! {
    static ref CLAIM_REGEX: Regex = Regex::new(r"#(\d+)\s*@\s*(\d+),(\d+):\s*(\d+)x(\d+)").expect("Claims regex does not compile");
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Claim {
    pub id: usize,
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl std::str::FromStr for Claim {
    type Err = failure::Error;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match CLAIM_REGEX.captures(s) {
            None => bail!("Claim not in expected format: {}", s),
            Some(captures) => {
                let id: usize = captures[1].parse()?;
                let x: usize = captures[2].parse()?;
                let y: usize = captures[3].parse()?;
                let width: usize = captures[4].parse()?;
                let height: usize = captures[5].parse()?;

                Ok(Claim { id, x, y, width, height })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claim_parsing_regex() {
        let captures = CLAIM_REGEX.captures(r"#1 @ 2,3: 4x5").unwrap();
        assert_eq!("1", &captures[1]);
        assert_eq!("2", &captures[2]);
        assert_eq!("3", &captures[3]);
        assert_eq!("4", &captures[4]);
        assert_eq!("5", &captures[5]);
    }

    #[test]
    fn claim_from_string() {
        let claim: Claim = r"#1 @ 2,3: 4x5".parse().expect("Failed to parse claim");

        assert_eq!(Claim { id: 1, x: 2, y: 3, width: 4, height: 5 }, claim);
    }
}