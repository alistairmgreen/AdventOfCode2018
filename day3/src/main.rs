use day3::*;
use failure::Error;

fn main() -> Result<(), Error> {
    let claims: Vec<Claim> = include_str!("puzzle_input.txt")
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<_>, Error>>()?;

    let multiply_claimed = count_squares_with_multiple_claims(&claims);

    println!(
        "{} squares fall within two or more claims.",
        multiply_claimed
    );

    for claim in &claims {
        if !claims.iter().filter(|other| *other != claim).any(|other| other.overlaps(claim)) {
            println!("Claim number {} does not overlap with any of the others.", claim.id);
        }
    }

    Ok(())
}
