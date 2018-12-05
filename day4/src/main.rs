use day4::*;

fn main() -> Result<(), EventParseError> {
    let events = read_events()?;

    let sleep_times = count_sleep_times(&events)?;

    if let Some((id, sleep_count)) = sleep_times
        .iter()
        .max_by_key(|(_, sleep)| sleep.iter().sum::<usize>())
    {
        println!("Guard #{} spends the most time asleep.", id);

        if let Some((minute, _)) = sleep_count.iter().enumerate().max_by_key(|(_, c)| *c) {
            println!(
                "He is most often asleep at {} minutes past midnight.",
                minute
            );

            println!("The answer to Part 1 is therefore {}", id * minute);
        }
    }

    if let Some((id, sleep_count)) = sleep_times
        .iter()
        .max_by_key(|(_, sleep)| sleep.iter().max().unwrap_or(&0))
    {
        if let Some((minute, times)) = sleep_count.iter().enumerate().max_by_key(|(_, c)| *c) {
            println!(
                "Guard #{} fell asleep {} times at {} minutes past midnight.",
                id, times, minute
            );

            println!("The answer to Part 2 is therefore {}", id * minute);
        }
    }

    Ok(())
}

fn read_events() -> Result<Vec<Event>, EventParseError> {
    let input = include_str!("puzzle_input.txt");

    let mut events = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Event>, EventParseError>>()?;

    events.sort();

    Ok(events)
}
