use std::fmt::Write;

fn main() {
    let puzzle_input = 990_941;
    let next_ten_scores = part1(puzzle_input);
    println!(
        "The scores of the next ten recipes are: {}",
        next_ten_scores
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

fn generate_recipes(improve_after: usize) -> Vec<u8> {
    let mut recipes: Vec<u8> = Vec::with_capacity(improve_after + 11);
    recipes.push(3);
    recipes.push(7);

    let mut elf_a = 0;
    let mut elf_b = 1;

    let required_recipes = improve_after + 10;
    while recipes.len() < required_recipes {
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
    let recipes = generate_recipes(improve_after);
    scores(&recipes, improve_after)
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
    fn test_generate_recipes() {
        let recipes = generate_recipes(9);
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
}
