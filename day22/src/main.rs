use day22::*;

fn main() {
    let target = (14,778);
    let depth = 11541;

    let total_risk = risk(target, depth);

    println!("The total risk is {}.", total_risk);
}
