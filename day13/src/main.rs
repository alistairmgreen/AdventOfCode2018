use day13::*;

fn main() {
    let input = include_str!("puzzle_input.txt");
    let (track, carts) = parse_input(&input);
    let (x, y) = simulate_until_collision(&track, carts.clone());
    println!("The first collision occurs at: {},{}", x, y);

    let (x, y) = simulate_until_only_one_cart_remains(&track, carts);
    println!("The last remaining cart is at {},{}", x, y);
}
