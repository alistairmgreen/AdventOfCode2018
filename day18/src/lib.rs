mod matrix;
pub use crate::matrix::Matrix;
use std::{fmt, mem::replace};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Acre {
    Open,
    Trees,
    Lumberyard,
}

impl Acre {
    pub fn try_parse(c: char) -> Option<Acre> {
        match c {
            '.' => Some(Acre::Open),
            '|' => Some(Acre::Trees),
            '#' => Some(Acre::Lumberyard),
            _ => None,
        }
    }
}

impl fmt::Display for Acre {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Acre::Open => write!(f, "."),
            Acre::Trees => write!(f, "|"),
            Acre::Lumberyard => write!(f, "#"),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Counts {
    trees: usize,
    lumberyards: usize,
}

fn count_neighbours(grid: &Matrix<Acre>, row: usize, column: usize) -> Counts {
    let mut trees = 0;
    let mut lumberyards = 0;

    let min_row = if row > 0 { row - 1 } else { 0 };
    let max_row = if row < grid.height() - 1 {
        row + 1
    } else {
        row
    };
    let min_column = if column > 0 { column - 1 } else { 0 };
    let max_column = if column < grid.width() - 1 {
        column + 1
    } else {
        column
    };

    // Clippy has a bug: it wants to change this loop to use methods that
    // exist on slices, but don't exist on Matrix.
    #[allow(clippy::needless_range_loop)]
    for y in min_row..=max_row {
        for x in min_column..=max_column {
            if !(y == row && x == column) {
                match grid[y][x] {
                    Acre::Lumberyard => {
                        lumberyards += 1;
                    }

                    Acre::Trees => {
                        trees += 1;
                    }

                    Acre::Open => {}
                }
            }
        }
    }

    Counts { trees, lumberyards }
}

fn next_generation(grid: &Matrix<Acre>) -> Matrix<Acre> {
    grid.map(|row, column, value| {
        let neighbours = count_neighbours(grid, row, column);

        match value {
            Acre::Open => {
                if neighbours.trees >= 3 {
                    Acre::Trees
                } else {
                    Acre::Open
                }
            }
            Acre::Trees => {
                if neighbours.lumberyards >= 3 {
                    Acre::Lumberyard
                } else {
                    Acre::Trees
                }
            }
            Acre::Lumberyard => {
                if neighbours.lumberyards > 0 && neighbours.trees > 0 {
                    Acre::Lumberyard
                } else {
                    Acre::Open
                }
            }
        }
    })
}

pub fn simulate(mut grid: Matrix<Acre>, generations: usize) -> Matrix<Acre> {
    for _ in 0..generations {
        let next = next_generation(&grid);
        replace(&mut grid, next);
    }

    grid
}

pub fn resource_value(grid: &Matrix<Acre>) -> usize {
    let mut trees = 0;
    let mut lumberyards = 0;

    for acre in grid.iter() {
        match acre {
            Acre::Trees => {
                trees += 1;
            }
            Acre::Lumberyard => {
                lumberyards += 1;
            }
            Acre::Open => {}
        }
    }

    trees * lumberyards
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = include_str!("example_input.txt");
        let elements = input.chars().filter_map(Acre::try_parse).collect();
        let grid = Matrix::from_vec(elements, 10);

        let result = simulate(grid, 10);
        assert_eq!(1147, resource_value(&result));
    }
}
