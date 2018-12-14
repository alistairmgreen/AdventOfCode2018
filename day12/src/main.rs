use std::{collections::{BTreeSet, HashMap}, mem};

fn main() {
    let initial_state = read_initial_state(".##.##...#.###..#.#..##..###..##...####.#...#.##....##.#.#...#...###.........##...###.....##.##.##");
    let rules: HashMap<Vec<Plant>, Plant> = include_str!("puzzle_rules.txt")
        .lines()
        .map(parse_rule)
        .collect();

    let final_state = simulate(initial_state, &rules, 20);
    let sum: i32 = final_state.iter().sum();

    println!("After 20 generations, the sum of all pot numbers with a living plant is: {}", sum);
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Plant {
    Alive,
    Dead,
}

fn read_initial_state(state: &str) -> BTreeSet<i32> {
    state
        .trim()
        .chars()
        .enumerate()
        .filter_map(|(index, item)| match item {
            '#' => Some(index as i32),
            _ => None,
        })
        .collect()
}

fn parse_rule(rule: &str) -> (Vec<Plant>, Plant) {
    let pattern: Vec<Plant> = rule.chars().map(|c| match c {
        '#' => Plant::Alive,
        _ => Plant::Dead,
    }).take(5).collect();

    let outcome = match rule.chars().last().unwrap() {
           '#' => Plant::Alive,
        _ => Plant::Dead,
    };

    (pattern, outcome)
}

fn next_generation(state: &BTreeSet<i32>, rules: &HashMap<Vec<Plant>, Plant>) -> BTreeSet<i32> {
    let mut next = BTreeSet::new();
    let first_plant = state.into_iter().take(1).last().unwrap_or(&0) - 3;
    let last_plant = state.into_iter().last().unwrap_or(&0) + 3;

    for index in first_plant..=last_plant {
        let chunk: Vec<Plant> = ((index - 2)..=(index + 2))
            .map(|n| {
                if state.contains(&n) {
                    Plant::Alive
                } else {
                    Plant::Dead
                }
            })
            .collect();
        
        if let Some(Plant::Alive) = rules.get(&chunk) {
            next.insert(index);
        }
    }

    next
}

fn simulate(mut state: BTreeSet<i32>, rules: &HashMap<Vec<Plant>, Plant>, generations: usize) -> BTreeSet<i32> {
    for _ in 0..generations {
        let next = next_generation(&state, &rules);
        mem::replace(&mut state, next);
    }

    state
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_simulation() {
        let initial_state = read_initial_state("#..#.#..##......###...###");
        let rules: HashMap<Vec<Plant>, Plant> = include_str!("example_rules.txt")
            .lines()
            .map(parse_rule)
            .collect();
        
        let final_state = simulate(initial_state, &rules, 20);
        assert_eq!(-2, *final_state.iter().take(1).last().unwrap());
        assert_eq!(34, *final_state.iter().last().unwrap());
        assert_eq!(325, final_state.iter().sum());
    }
}