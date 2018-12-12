mod parsing;
mod vectors;
use crate::parsing::*;
use crate::vectors::Vector2D;
use lodepng;
use rgb::RGBA8;
use std::{
    collections::HashSet,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() -> Result<(), Box<dyn Error>> {
    let vectors = read_input(&"puzzle_input.txt")?;

    let message_time = find_tightest_cluster(&vectors);
    println!("Message forms at t = {} seconds.", message_time);

    let positions = calculate_positions(&vectors, message_time);

    save_image(&positions, &"message.png")?;

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

fn calculate_positions(
    initial_position_velocity: &[(Vector2D, Vector2D)],
    time: i64,
) -> Vec<Vector2D> {
    initial_position_velocity
        .iter()
        .cloned()
        .map(|(position, velocity)| position + velocity * time)
        .collect()
}

fn bounds(points: &[Vector2D]) -> (i64, i64, i64, i64) {
    let mut min_x = points[0].x;
    let mut min_y = points[0].y;
    let mut max_x = points[0].x;
    let mut max_y = points[0].y;

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

fn save_image<P: AsRef<Path>>(points: &[Vector2D], filename: P) -> Result<(), lodepng::Error> {
    let (min_x, min_y, max_x, max_y) = bounds(points);
    let width = (max_x - min_x + 1) as usize;
    let height = (max_y - min_y + 1) as usize;
    let pixel_count = width * height;
    let mut image = Vec::with_capacity(pixel_count);

    let point_set: HashSet<Vector2D> = points.iter().cloned().collect();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let pixel = if point_set.contains(&Vector2D { x, y }) {
                RGBA8::new(0, 0, 0, 255)
            } else {
                RGBA8::new(255, 255, 255, 255)
            };
            image.push(pixel);
        }
    }

    lodepng::encode32_file(filename, &image, width, height)
}

/// Calculates the mean square distance between the satellites and their centre of mass.
/// This gives an indication of when they are most tightly clustered (and hence approximately
/// when the message should appear).
fn spread(points: &[Vector2D]) -> i64 {
    let centre_of_mass: Vector2D = points.iter().cloned().sum::<Vector2D>() / points.len() as i64;

    points
        .iter()
        .cloned()
        .map(|point| (point - centre_of_mass).abs_square())
        .sum()
}

/// Finds the time at which the satellites form the tightest cluster.
/// This is likely to be close to the time where the message appears.
fn find_tightest_cluster(vectors: &[(Vector2D, Vector2D)]) -> i64 {
     let mut previous_spread = spread(&calculate_positions(&vectors, 0));
    let max_t = 100_000;

    for t in 1..=max_t {
        let current_spread = spread(&calculate_positions(&vectors, t));
        if current_spread > previous_spread {
            return t - 1;
        }

        previous_spread = current_spread;
    }

    max_t
}
