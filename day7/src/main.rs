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

    let sequence = find_order(steps, prerequisites);

    println!("The steps must be performed in order:\n{}", sequence);
}

fn find_order(mut steps: HashSet<char>, mut prerequisites: HashMap<char, HashSet<char>>) -> String {
    let mut sequence = String::with_capacity(steps.len());

    while !steps.is_empty() {
        let mut candidates: Vec<char> = steps
            .iter()
            .cloned()
            .filter(|step| match prerequisites.get(step) {
                None => true,
                Some(required) => !required.iter().any(|r| steps.contains(r)),
            })
            .collect();

        assert!(!candidates.is_empty());

        candidates.sort();

        let step = candidates[0];
        sequence.push(step);
        steps.remove(&step);
        prerequisites.remove(&step);
    }

    sequence
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_order() {
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

        let sequence = find_order(steps, prerequisites);

        assert_eq!("CABDFE".to_string(), sequence);
    }
}
