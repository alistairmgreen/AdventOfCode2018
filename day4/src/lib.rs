use chrono::NaiveDateTime;
use std::{cmp, fmt, num::ParseIntError, str::FromStr};

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
}

impl fmt::Display for EventParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EventParseError::InvalidDate => write!(f, "Cannot parse date"),
            EventParseError::InvalidType => write!(f, "Unexpected event type"),
            EventParseError::InvalidId => write!(f, "Cannot parse guard ID"),
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
}
