use day3::Claim;
use failure::Error;

fn main() -> Result<(), Error> {
    let claims: Vec<Claim> = include_str!("puzzle_input.txt")
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<_>, Error>>()?;

    let grid_width = claims
        .iter()
        .map(|claim| claim.x + claim.width)
        .max()
        .unwrap_or(0);

    let grid_height = claims
        .iter()
        .map(|claim| claim.y + claim.height)
        .max()
        .unwrap_or(0);

    println!("{} claims loaded", claims.len());
    println!("Grid measures {} x {} inches.", grid_width, grid_height);

    Ok(())
}
