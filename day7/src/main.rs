use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("puzzle_input.txt");

    let mut steps = HashSet::new();
    let mut prerequisites = HashMap::new();

    for line in input.lines() {
        let characters: Vec<char> = line.chars().collect();
        let prerequisite = characters[5];
        let step = characters[36];

        steps.insert(step);
        steps.insert(prerequisite);

        prerequisites
            .entry(step)
            .or_insert_with(HashSet::new)
            .insert(prerequisite);
    }

    println!("Part 1:");

    let sequence = find_order(steps.clone(), prerequisites.clone());

    println!("The steps must be performed in order:\n{}", sequence);

    println!("Part 2:");
    let time_required = parallel_construction(steps, prerequisites, 5, time_for_step);
    println!("Five workers can assemble the sleigh in {} seconds.", time_required);
}

fn find_order(mut steps: HashSet<char>, mut prerequisites: HashMap<char, HashSet<char>>) -> String {
    let step_count = steps.len();
    let mut sequence = String::with_capacity(step_count);
    let mut completed_steps = HashSet::with_capacity(step_count);

    while !steps.is_empty() {
        let step = next_step(&steps, &prerequisites, &completed_steps)
            .expect("Cannot proceed with any of the remaining steps");
        sequence.push(step);
        completed_steps.insert(step);
        steps.remove(&step);
        prerequisites.remove(&step);
    }

    sequence
}

fn next_step(
    remaining_steps: &HashSet<char>,
    prerequisites: &HashMap<char, HashSet<char>>,
    completed_steps: &HashSet<char>,
) -> Option<char> {
    let mut candidates: Vec<char> = remaining_steps
        .iter()
        .cloned()
        .filter(|step| match prerequisites.get(step) {
            None => true,
            Some(required) => required.iter().all(|r| completed_steps.contains(r)),
        })
        .collect();

    candidates.sort();

    match candidates.get(0) {
        Some(&letter) => Some(letter),
        None => None,
    }
}

fn parallel_construction(
    mut steps: HashSet<char>,
    mut prerequisites: HashMap<char, HashSet<char>>,
    num_workers: usize,
    step_time: impl Fn(char) -> usize,
) -> usize {
    let step_count = steps.len();
    let mut completed_steps: HashSet<char> = HashSet::with_capacity(step_count);
    let mut seconds = 0;
    let mut worker_tasks: Vec<Option<char>> = vec![None; num_workers];
    let mut worker_remaining_time: Vec<usize> = vec![0; num_workers];

    while completed_steps.len() < step_count {
        // Have any workers finished what they are doing?
        for n in 0..num_workers {
            if let Some(step_in_progress) = worker_tasks[n] {
                worker_remaining_time[n] -= 1;
                if worker_remaining_time[n] == 0 {
                    completed_steps.insert(step_in_progress);
                    worker_tasks[n] = None;
                }
            }
        }

        // If any tasks remain unstarted, allocate them to idle workers.

        if !steps.is_empty() {
            for n in 0..num_workers {
                if worker_tasks[n].is_none() {
                    if let Some(next_task) = next_step(&steps, &prerequisites, &completed_steps) {
                        worker_tasks[n] = Some(next_task);
                        worker_remaining_time[n] = step_time(next_task);
                        steps.remove(&next_task);
                        prerequisites.remove(&next_task);
                    }
                }
            }
        }

        seconds += 1;
    }

    seconds - 1
}

fn time_for_step(letter: char) -> usize {
    (letter as u8 - b'A' + 61) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> (HashSet<char>, HashMap<char, HashSet<char>>) {
        let mut steps = HashSet::with_capacity(6);
        for step in b'A'..=b'F' {
            steps.insert(step as char);
        }

        let mut prerequisites = HashMap::with_capacity(7);

        let mut a_prerequisites = HashSet::with_capacity(1);
        a_prerequisites.insert('C');
        prerequisites.insert('A', a_prerequisites);

        let mut b_prerequisites = HashSet::with_capacity(1);
        b_prerequisites.insert('A');
        prerequisites.insert('B', b_prerequisites);

        let mut d_prerequisites = HashSet::with_capacity(1);
        d_prerequisites.insert('A');
        prerequisites.insert('D', d_prerequisites);

        let mut e_prerequisites = HashSet::with_capacity(3);
        e_prerequisites.insert('B');
        e_prerequisites.insert('D');
        e_prerequisites.insert('F');
        prerequisites.insert('E', e_prerequisites);

        let mut f_prerequisites = HashSet::with_capacity(1);
        f_prerequisites.insert('C');
        prerequisites.insert('F', f_prerequisites);

        (steps, prerequisites)
    }

    #[test]
    fn test_find_order() {
        let (steps, prerequisites) = example_input();
        let sequence = find_order(steps, prerequisites);

        assert_eq!("CABDFE".to_string(), sequence);
    }

    fn test_step_time(letter: char) -> usize {
        (letter as u8 - b'A' + 1) as usize
    }

    #[test]
    fn test_part2() {
        let (steps, prerequisites) = example_input();
        let time = parallel_construction(steps, prerequisites, 2, test_step_time);
        assert_eq!(15, time);
    }

    #[test]
    fn test_time_for_step() {
        assert_eq!(61, time_for_step('A'));
        assert_eq!(86, time_for_step('Z'));
    }
}
