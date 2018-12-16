use day13::*;

fn main() {
    let input = include_str!("puzzle_input.txt");
    let (track, carts) = parse_input(&input);
    let (x, y) = simulate_until_collision(&track, carts);
    println!("The first collision occurs at: {},{}", x, y);
}
