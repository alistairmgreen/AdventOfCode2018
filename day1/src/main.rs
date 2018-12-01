use std::{
    collections::HashSet,
    io::{BufRead, BufReader},
    error::Error,
    fs::File,
};

fn main() -> Result<(), Box<dyn Error>> {
    let deltas = read_deltas()?;
    let final_frequency: i32 = deltas.iter().sum();

    println!("The final frequency is {}.", final_frequency);

    let mut frequency = 0;
    let mut past_frequencies = HashSet::new();
    past_frequencies.insert(frequency);

    for delta in deltas.iter().cycle() {
        frequency += delta;
        if !past_frequencies.insert(frequency) {
            break;
        }
    }

    println!("The first repeated frequency value is {}.", frequency);

    Ok(())
}

fn read_deltas() -> Result<Vec<i32>, Box<dyn Error>> {
    let file = File::open("puzzle_input.txt")?;
    let reader = BufReader::new(file);
    let mut deltas = Vec::new();

    for line in reader.lines() {
        let delta = line?.trim().parse()?;
        deltas.push(delta);
    }

    Ok(deltas)
}