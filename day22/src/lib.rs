use std::collections::HashMap;

type Coordinate = (i32, i32);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum RegionType {
    Rocky,
    Wet,
    Narrow,
}

impl RegionType {
    pub fn risk(&self) -> i32 {
        match self {
            RegionType::Rocky => 0,
            RegionType::Wet => 1,
            RegionType::Narrow => 2,
        }
    }

    fn of(position: Coordinate, target: Coordinate, depth: i32, cache: &mut HashMap<Coordinate, i32>) -> RegionType {
        let index = geologic_index(position, target, depth, cache);
        
        match erosion(index, depth) % 3 {
            0 => RegionType::Rocky,
            1 => RegionType::Wet,
            2 => RegionType::Narrow,
            _ => unreachable!("Modulo 3 arithmetic cannot produce value bigger than 2")
        }
    }
}

fn geologic_index(position: Coordinate, target: Coordinate, depth: i32, cache: &mut HashMap<Coordinate, i32>) -> i32 {
    if let Some(index) = cache.get(&position) {
        return *index;
    }

    let index = match position {
        (0, 0) => 0,
        (x, y) if (x, y) == target => 0,
        (0, y) => y * 48271,
        (x, 0) => x * 16807,
        (x, y) => erosion(geologic_index((x -1, y), target, depth, cache), depth) * erosion(geologic_index((x, y -1), target, depth, cache), depth)
    };

    cache.insert(position, index);
    index
}

fn erosion(index: i32, depth: i32) -> i32 {
    (index + depth) % 20183
}

pub fn risk(target: Coordinate, depth: i32) -> i32 {
    let mut cache = HashMap::new();
    let mut risk_value = 0;

    for x in 0..=target.0 {
        for y in 0..=target.1 {
            let region = RegionType::of((x, y), target, depth, &mut cache);
            risk_value += region.risk();
        }
    }

    risk_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_risk() {
        assert_eq!(114, risk((10, 10), 510));
    }
}