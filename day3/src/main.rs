use day3::*;
use failure::Error;

fn main() -> Result<(), Error> {
    let claims: Vec<Claim> = include_str!("puzzle_input.txt")
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<_>, Error>>()?;

    let multiply_claimed = count_squares_with_multiple_claims(&claims);

    println!("{} squares fall within two or more claims.", multiply_claimed);

    Ok(())
}
