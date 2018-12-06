#[macro_use]
extern crate lazy_static;

mod input;

use std::collections::{HashMap, HashSet};

type Point = (i32, i32);

fn main() {
    let largest = find_largest_area(&input::PUZZLE_INPUT);
    println!("The largest area is {}.", largest);

    let part2 = part2_region_size(&input::PUZZLE_INPUT, 10_000);
    println!(
        "Part 2: The region with total distance < 10,000 has area {}.",
        part2
    );
}

/// Calculates the distance between two points in "Manhattan" geometry.
fn manhattan(from: Point, to: Point) -> i32 {
    i32::abs(to.0 - from.0) + i32::abs(to.1 - from.1)
}

fn total_distance(from: Point, points: &[Point]) -> i32 {
    points.iter().map(|&point| manhattan(from, point)).sum()
}

/// Finds the closest point to the given coordinates, if there is a unique answer.
/// Returns None if there is a tie.
fn find_closest(position: Point, other_points: &[Point]) -> Option<Point> {
    let distances: Vec<i32> = other_points
        .iter()
        .map(|&p| manhattan(position, p))
        .collect();
    let shortest = *distances.iter().min()?;

    let closest_points: Vec<Point> = other_points
        .iter()
        .zip(distances.iter())
        .filter_map(|(&point, &distance)| {
            if distance == shortest {
                Some(point)
            } else {
                None
            }
        })
        .collect();

    if closest_points.len() == 1 {
        Some(closest_points[0])
    } else {
        None
    }
}

fn bounds(points: &[Point]) -> (Point, Point) {
    assert!(!points.is_empty());

    let mut x_min = points[0].0;
    let mut x_max = x_min;
    let mut y_min = points[0].1;
    let mut y_max = y_min;

    for (x, y) in points {
        if *x < x_min {
            x_min = *x;
        }

        if *x > x_max {
            x_max = *x;
        }

        if *y < y_min {
            y_min = *y;
        }

        if *y > y_max {
            y_max = *y;
        }
    }

    ((x_min, y_min), (x_max, y_max))
}

fn find_largest_area(points: &[Point]) -> i32 {
    let ((left, top), (right, bottom)) = bounds(points);

    let mut areas = HashMap::new();
    let mut infinite_areas = HashSet::new();

    // Look at the rows and columns *outside* the area bounded by the points.
    // Any point closest to these must have an infinite area associated with it.

    for x in left - 1..=right + 1 {
        if let Some(nearest) = find_closest((x, top - 1), points) {
            infinite_areas.insert(nearest);
        }

        if let Some(nearest) = find_closest((x, bottom + 1), points) {
            infinite_areas.insert(nearest);
        }
    }

    for y in top..=bottom {
        if let Some(nearest) = find_closest((left - 1, y), points) {
            infinite_areas.insert(nearest);
        }

        if let Some(nearest) = find_closest((right + 1, y), points) {
            infinite_areas.insert(nearest);
        }
    }

    for x in left..=right {
        for y in top..=bottom {
            if let Some(nearest) = find_closest((x, y), points) {
                if !infinite_areas.contains(&nearest) {
                    *areas.entry(nearest).or_insert(0) += 1;
                }
            }
        }
    }

    areas.values().cloned().max().unwrap_or(0)
}

fn part2_region_size(points: &[Point], max_distance: i32) -> usize {
    let ((left, top), (right, bottom)) = bounds(points);

    let mut region_size = 0;

    for x in (left - max_distance)..=(right + max_distance) {
        for y in (top - max_distance)..=(bottom + max_distance) {
            if total_distance((x, y), points) < max_distance {
                region_size += 1;
            }
        }
    }

    region_size
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::EXAMPLE_INPUT;

    #[test]
    fn test_find_closest() {
        assert_eq!(Some((1, 1)), find_closest((0, 0), &EXAMPLE_INPUT));

        assert_eq!(None, find_closest((1, 4), &EXAMPLE_INPUT));
    }

    #[test]
    fn largest_area_part_1_example() {
        assert_eq!(17, find_largest_area(&EXAMPLE_INPUT));
    }

    #[test]
    fn total_distance_example() {
        assert_eq!(30, total_distance((4, 3), &EXAMPLE_INPUT));
    }

    #[test]
    fn part2_example() {
        assert_eq!(16, part2_region_size(&EXAMPLE_INPUT, 32));
    }
}
