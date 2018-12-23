use day23::*;

fn main() -> Result<(), Error> {
    let input = include_str!("puzzle_input.txt");
    let mut nanobots = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Nanobot>, Error>>()?;

    nanobots.sort_unstable_by(|bot1, bot2| bot2.radius.cmp(&bot1.radius));
    let strongest_bot = &nanobots[0];
    println!("Strongest bot: {:?}", strongest_bot);

    let in_range = nanobots
        .iter()
        .filter(|bot| bot.manhattan_distance(strongest_bot) <= strongest_bot.radius)
        .count();

    println!("{} nanobots are in range.", in_range);
    
    let most_in_range = nanobots.iter()
        .map(|bot| nanobots.iter().filter(|bot2| bot.manhattan_distance(bot2) <= bot.radius).count())
        .max()
        .unwrap();

    println!("A bot exists within range of {} other bots.", most_in_range);

    Ok(())
}
