use day18::*;

fn main() {
    let input = include_str!("puzzle_input.txt");
    let elements = input.chars().filter_map(Acre::try_parse).collect();
    let grid = Matrix::from_vec(elements, 50);

    let result = simulate(grid, 10);

    println!("After 10 minutes: resource value = {}\n", resource_value(&result));
    println!("{}", result);
}
