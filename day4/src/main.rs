use day4::*;

fn main() -> Result<(), EventParseError> {
    let events = read_events()?;
    let count = events.len();

    println!("{} events loaded.", count);
    println!("The first event is:\n{:#?}", events[0]);
    println!("The last event is:\n{:#?}", events[count - 1]);

    Ok(())
}

fn read_events() -> Result<Vec<Event>, EventParseError> {
    let input = include_str!("puzzle_input.txt");

    let mut events = input.lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Event>, EventParseError>>()?;

    events.sort();

    Ok(events)
}
