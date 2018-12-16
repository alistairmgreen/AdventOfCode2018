use std::fmt::Write;

fn main() {
    let puzzle_input = 990_941;
    let next_ten_scores = part1(puzzle_input);
    println!(
        "The scores of the next ten recipes are: {}",
        next_ten_scores
    );

    let part2_solution = part2(&[9, 9, 0, 9, 4, 1]);
    println!(
        "{} recipes appear to the left of the sequence 990941.",
        part2_solution
    );
}

fn create_new_recipes(recipes: &mut Vec<u8>, current_a: usize, current_b: usize) {
    let sum = recipes[current_a] + recipes[current_b];
    if sum < 10 {
        recipes.push(sum);
    } else {
        let units = sum % 10;
        let tens = (sum - units) / 10;
        recipes.push(tens);
        recipes.push(units);
    }
}

fn generate_recipes_until(end_condition: impl Fn(&[u8]) -> bool) -> Vec<u8> {
    let mut recipes = Vec::new();
    recipes.push(3);
    recipes.push(7);

    let mut elf_a = 0;
    let mut elf_b = 1;

    while !end_condition(&recipes) {
        create_new_recipes(&mut recipes, elf_a, elf_b);

        let recipe_count = recipes.len();
        elf_a = (elf_a + 1 + recipes[elf_a] as usize) % recipe_count;
        elf_b = (elf_b + 1 + recipes[elf_b] as usize) % recipe_count;
    }

    recipes
}

fn scores(recipes: &[u8], improve_after: usize) -> String {
    let mut scores = String::with_capacity(10);

    for recipe in recipes.iter().skip(improve_after).take(10) {
        write!(&mut scores, "{}", recipe).expect("Failed to write scores to string buffer");
    }

    scores
}

fn part1(improve_after: usize) -> String {
    let recipes = generate_recipes_until(|recipes| recipes.len() >= improve_after + 10);
    scores(&recipes, improve_after)
}

fn part2(sought_recipes: &[u8]) -> usize {
    let length_of_sought = sought_recipes.len();
    let recipes = generate_recipes_until(|recipes| {
        let recipe_length = recipes.len();

        if recipe_length == length_of_sought {
            recipes == sought_recipes
        } else if recipe_length > length_of_sought {
            recipes[(recipe_length - length_of_sought)..] == *sought_recipes
                || recipes[(recipe_length - length_of_sought - 1)..recipe_length - 1]
                    == *sought_recipes
        } else {
            false
        }
    });

    // There may or may not be exactly one recipe beyond the ones we are looking for.
    let recipe_length = recipes.len();
    if recipes[(recipe_length - length_of_sought)..] == *sought_recipes {
        recipe_length - length_of_sought
    } else {
        recipe_length - length_of_sought - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new_recipes() {
        let mut recipes = vec![3, 7];
        create_new_recipes(&mut recipes, 0, 1);
        assert_eq!(vec![3, 7, 1, 0], recipes);
    }

    #[test]
    fn test_generate_recipes_until() {
        let recipes = generate_recipes_until(|recipes| recipes.len() >= 19);
        let expected = vec![3, 7, 1, 0, 1, 0, 1, 2, 4, 5, 1, 5, 8, 9, 1, 6, 7, 7, 9];
        assert_eq!(expected, recipes);
    }

    #[test]
    fn test_part1_examples() {
        assert_eq!("5158916779".to_string(), part1(9));
        assert_eq!("0124515891".to_string(), part1(5));
        assert_eq!("9251071085".to_string(), part1(18));
        assert_eq!("5941429882".to_string(), part1(2018));
    }

    #[test]
    fn test_part2_examples() {
        assert_eq!(9, part2(&[5, 1, 5, 8, 9]));
        assert_eq!(9, part2(&[5, 1, 5, 8, 9, 1, 6, 7, 7, 9]));
        assert_eq!(5, part2(&[0, 1, 2, 4, 5]));
        assert_eq!(18, part2(&[9, 2, 5, 1, 0]));
        assert_eq!(2018, part2(&[5, 9, 4, 1, 4]));
        assert_eq!(2, part2(&[1]));
        assert_eq!(2, part2(&[1, 0]));
    }
}
