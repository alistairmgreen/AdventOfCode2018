use std::{
    io::{BufRead, BufReader},
    error::Error,
    fs::File,
};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("puzzle_input.txt")?;
    let reader = BufReader::new(file);

    let mut frequency = 0;

    for line in reader.lines() {
        let line = line?;
        let shift: i32 = line.trim().parse()?;
        frequency += shift;
    }

    println!("The final frequency is {}.", frequency);

    Ok(())
}
