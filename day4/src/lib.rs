use chrono::{NaiveDateTime, Timelike};
use std::{cmp, collections::HashMap, fmt, num::ParseIntError, str::FromStr};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum EventType {
    GuardChange(usize),
    FallAsleep,
    Awake,
}

impl FromStr for EventType {
    type Err = EventParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("Guard #") {
            let id: String = s.chars().skip(7).take_while(|c| c.is_digit(10)).collect();
            let id: usize = id.parse()?;

            Ok(EventType::GuardChange(id))
        } else if s.contains("falls asleep") {
            Ok(EventType::FallAsleep)
        } else if s.contains("wakes up") {
            Ok(EventType::Awake)
        } else {
            Err(EventParseError::InvalidType)
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Event {
    pub time: NaiveDateTime,
    pub event_type: EventType,
}

impl FromStr for Event {
    type Err = EventParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let time = match s.get(1..17) {
            Some(date) => NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M")?,
            None => return Err(EventParseError::InvalidDate),
        };

        let event_type: EventType = match s.get(19..) {
            Some(e) => e.parse()?,
            None => return Err(EventParseError::InvalidType),
        };

        Ok(Event { time, event_type })
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Event) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Event) -> cmp::Ordering {
        self.time.cmp(&other.time)
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum EventParseError {
    InvalidDate,
    InvalidType,
    InvalidId,
    UnspecifiedGuard,
}

impl fmt::Display for EventParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EventParseError::InvalidDate => write!(f, "Cannot parse date"),
            EventParseError::InvalidType => write!(f, "Unexpected event type"),
            EventParseError::InvalidId => write!(f, "Cannot parse guard ID"),
            EventParseError::UnspecifiedGuard => {
                write!(f, "Wake or sleep event precedes first guard change")
            }
        }
    }
}

impl std::error::Error for EventParseError {}

impl From<chrono::format::ParseError> for EventParseError {
    fn from(_: chrono::format::ParseError) -> EventParseError {
        EventParseError::InvalidDate
    }
}

impl From<ParseIntError> for EventParseError {
    fn from(_: ParseIntError) -> EventParseError {
        EventParseError::InvalidId
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Wakefulness {
    Awake,
    Asleep,
}

pub fn count_sleep_times(events: &[Event]) -> Result<HashMap<usize, Vec<usize>>, EventParseError> {
    let mut sleep_times = HashMap::new();

    let mut guard_id = match events.get(0) {
        Some(event) => match event.event_type {
            EventType::GuardChange(id) => id,
            _ => return Err(EventParseError::UnspecifiedGuard),
        },
        None => return Ok(sleep_times),
    };

    let mut sleep_state = vec![Wakefulness::Awake; 60];

    for event in events.iter().skip(1) {
        match event.event_type {
            EventType::GuardChange(id) => {
                update_sleep_times(guard_id, &sleep_state, &mut sleep_times);
                guard_id = id;
                sleep_state.iter_mut().for_each(|s| {
                    *s = Wakefulness::Awake;
                });
            }

            EventType::FallAsleep => {
                sleep_state
                    .iter_mut()
                    .skip(event.time.minute() as usize)
                    .for_each(|s| {
                        *s = Wakefulness::Asleep;
                    });
            }

            EventType::Awake => {
                sleep_state
                    .iter_mut()
                    .skip(event.time.minute() as usize)
                    .for_each(|s| {
                        *s = Wakefulness::Awake;
                    });
            }
        }
    }

    update_sleep_times(guard_id, &sleep_state, &mut sleep_times);

    Ok(sleep_times)
}

fn update_sleep_times(
    guard_id: usize,
    sleep_state: &[Wakefulness],
    sleep_times: &mut HashMap<usize, Vec<usize>>,
) {
    let sleep_count = sleep_times.entry(guard_id).or_insert_with(|| vec![0; 60]);

    for (count, wakefulness) in sleep_count.iter_mut().zip(sleep_state.iter()) {
        if *wakefulness == Wakefulness::Asleep {
            *count += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, Timelike};

    #[test]
    fn parse_guard_change() {
        let guard_change: EventType = "Guard #2851 begins shift".parse().unwrap();
        assert_eq!(EventType::GuardChange(2851), guard_change);
    }

    #[test]
    fn parse_falls_asleep() {
        let asleep: EventType = "falls asleep".parse().unwrap();
        assert_eq!(EventType::FallAsleep, asleep);
    }

    #[test]
    fn parse_wakes_up() {
        let wake: EventType = "wakes up".parse().unwrap();
        assert_eq!(EventType::Awake, wake);
    }

    #[test]
    fn parse_full_input_record() {
        let event: Event = "[1518-03-11 23:47] Guard #1223 begins shift"
            .parse()
            .unwrap();

        assert_eq!(EventType::GuardChange(1223), event.event_type);
        assert_eq!(1518, event.time.year());
        assert_eq!(3, event.time.month());
        assert_eq!(11, event.time.day());
        assert_eq!(23, event.time.hour());
        assert_eq!(47, event.time.minute());
    }

    #[test]
    fn sleep_times_example() {
        let events: Vec<Event> = include_str!("example_input.txt")
            .lines()
            .map(|line| line.parse().unwrap())
            .collect();
        
        let sleep_times = count_sleep_times(&events).unwrap();

        let guard10 = &sleep_times[&10];

        assert_eq!(0, guard10[0]);
        assert_eq!(1, guard10[5]);
        assert_eq!(2, guard10[24]);

        let guard99 = &sleep_times[&99];

        assert_eq!(3, guard99[45]);
    }
}
