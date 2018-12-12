mod parsing;
mod vectors;
use crate::parsing::*;
use crate::vectors::Vector2D;
use std::{
    collections::HashSet,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() -> Result<(), Box<dyn Error>> {
    let vectors = read_input(&"example_input.txt")?;

    let positions: Vec<Vector2D> = vectors
        .iter()
        .cloned()
        .map(|(position, velocity)| position + velocity * 3)
        .collect();
    let output = render(&positions);
    println!("{}\n", output);

    Ok(())
}

fn read_input<T: AsRef<Path>>(filename: &T) -> Result<Vec<(Vector2D, Vector2D)>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut positions_velocities: Vec<(Vector2D, Vector2D)> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let vectors = parse_input(&line)?;
        positions_velocities.push(vectors);
    }

    Ok(positions_velocities)
}

fn bounds(points: &[Vector2D]) -> (i32, i32, i32, i32) {
    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;

    for point in points {
        if point.x < min_x {
            min_x = point.x;
        }

        if point.y < min_y {
            min_y = point.y;
        }

        if point.x > max_x {
            max_x = point.x;
        }

        if point.y > max_y {
            max_y = point.y;
        }
    }

    (min_x, min_y, max_x, max_y)
}

fn render(points: &[Vector2D]) -> String {
    let (min_x, min_y, max_x, max_y) = bounds(points);
    let capacity = ((max_x - min_x + 1) * (max_y - min_y)) as usize;
    let mut output = String::with_capacity(capacity);

    let point_set: HashSet<Vector2D> = points.iter().cloned().collect();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let character = if point_set.contains(&Vector2D { x, y }) {
                '#'
            } else {
                '.'
            };
            output.push(character);
        }
        output.push('\n');
    }

    output
}
