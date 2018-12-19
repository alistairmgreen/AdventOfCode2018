use day18::*;
use std::{collections::HashMap, mem::replace};

fn main() {
    let input = include_str!("puzzle_input.txt");
    let elements = input.chars().filter_map(Acre::try_parse).collect();
    let grid = Matrix::from_vec(elements, 50);

    let result = simulate(grid.clone(), 10);

    println!("After 10 minutes: resource value = {}\n", resource_value(&result));
    println!("{}", result);

    // Part 2
    if let Some(t) = find_equivalent_of_billionth_generation(grid.clone()) {
        let billionth = simulate(grid, t);
        println!("Resource value: {}", resource_value(&billionth));
    }
}

fn find_equivalent_of_billionth_generation(mut grid: Matrix<Acre>) -> Option<usize> {
let mut past_generations = HashMap::new();
    past_generations.insert(grid.clone(), 0);
    
    for t in 0..=10_000 {
        let next = next_generation(&grid);
        replace(&mut grid, next);

        if let Some(generation) = past_generations.get(&grid) {
            let cycle_time: usize = t - generation;
            println!("Generation {} is the same as generation {}.", t, generation);
            println!("The pattern repeats every {} generations.", cycle_time);
            let target: usize = 1_000_000_000;
            let required = ((target - generation) % cycle_time) + generation;
            println!("Therefore generation {} is the same as generation {}.", target, required);
            return Some(required);
        }

        past_generations.insert(grid.clone(), t);
    }

    None
}