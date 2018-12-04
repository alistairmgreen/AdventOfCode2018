#[macro_use]
extern crate failure;

#[macro_use]
extern crate lazy_static;

use regex::Regex;

lazy_static! {
    static ref CLAIM_REGEX: Regex = Regex::new(r"#(\d+)\s*@\s*(\d+),(\d+):\s*(\d+)x(\d+)")
        .expect("Claims regex does not compile");
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Claim {
    pub id: usize,
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl Claim {
    pub fn overlaps(&self, other: &Claim) -> bool {
        let self_right = self.x + self.width - 1;
        let other_right = other.x + other.width - 1;
        let self_bottom = self.y + self.height - 1;
        let other_bottom = other.y + other.height - 1;

        (
            (self.x >= other.x && self.x <= other_right)
            || (self_right >= other.x && self_right <= other_right)
            || (other.x >= self.x && other.x <= self_right)
            || (other_right >= self.x && other_right <= self_right)
        )
            && (
                (self.y >= other.y && self.y <= other_bottom)
                || (self_bottom >= other.y && self_bottom <= other_bottom)
                || (other.y >= self.y && other.y <= self_bottom)
                || (other_bottom >= self.y && other_bottom <= self_bottom)
        )
    }
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

                Ok(Claim {
                    id,
                    x,
                    y,
                    width,
                    height,
                })
            }
        }
    }
}

/// Finds the number of squares in a given row that fall within two or more claims.
fn count_overlaps_in_row(row: usize, width: usize, claims: &[Claim]) -> usize {
    let mut claim_counts = vec![0; width];
    let claims_in_row = claims
        .iter()
        .filter(|claim| claim.y <= row && claim.y + claim.height > row);

    for claim in claims_in_row {
        claim_counts
            .iter_mut()
            .skip(claim.x)
            .take(claim.width)
            .for_each(|count| {
                *count += 1;
            });
    }

    claim_counts.iter().filter(|&&c| c > 1).count()
}

pub fn count_squares_with_multiple_claims(claims: &[Claim]) -> usize {
    let grid_width = claims
        .iter()
        .map(|claim| claim.x + claim.width)
        .max()
        .unwrap_or(0)
        + 1;

    let grid_height = claims
        .iter()
        .map(|claim| claim.y + claim.height)
        .max()
        .unwrap_or(0)
        + 1;

    (0..grid_height)
        .map(|row| count_overlaps_in_row(row, grid_width, claims))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref EXAMPLE_CLAIMS: Vec<Claim> = vec![
            Claim {
                id: 1,
                x: 1,
                y: 3,
                width: 4,
                height: 4
            },
            Claim {
                id: 2,
                x: 3,
                y: 1,
                width: 4,
                height: 4
            },
            Claim {
                id: 3,
                x: 5,
                y: 5,
                width: 2,
                height: 2
            },
        ];
    }

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

        assert_eq!(
            Claim {
                id: 1,
                x: 2,
                y: 3,
                width: 4,
                height: 5
            },
            claim
        );
    }

    #[test]
    fn test_count_overlaps_in_row() {
        assert_eq!(0, count_overlaps_in_row(0, 8, &EXAMPLE_CLAIMS));
        assert_eq!(0, count_overlaps_in_row(1, 8, &EXAMPLE_CLAIMS));
        assert_eq!(0, count_overlaps_in_row(2, 8, &EXAMPLE_CLAIMS));
        assert_eq!(2, count_overlaps_in_row(3, 8, &EXAMPLE_CLAIMS));
        assert_eq!(2, count_overlaps_in_row(4, 8, &EXAMPLE_CLAIMS));
        assert_eq!(0, count_overlaps_in_row(5, 8, &EXAMPLE_CLAIMS));
        assert_eq!(0, count_overlaps_in_row(6, 8, &EXAMPLE_CLAIMS));
        assert_eq!(0, count_overlaps_in_row(7, 8, &EXAMPLE_CLAIMS));
    }

    #[test]
    fn example_for_part_1() {
        assert_eq!(4, count_squares_with_multiple_claims(&EXAMPLE_CLAIMS));
    }

    #[test]
    fn example_claim_overlaps() {
        assert!(EXAMPLE_CLAIMS[0].overlaps(&EXAMPLE_CLAIMS[1]));
        assert!(EXAMPLE_CLAIMS[1].overlaps(&EXAMPLE_CLAIMS[0]));

        assert!(!EXAMPLE_CLAIMS[0].overlaps(&EXAMPLE_CLAIMS[2]));
        assert!(!EXAMPLE_CLAIMS[1].overlaps(&EXAMPLE_CLAIMS[2]));
    }
}
