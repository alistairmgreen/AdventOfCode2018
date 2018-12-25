use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let input = include_str!("puzzle_input.txt");
    let constellations = count_constellations_in_list(&input)?;

    println!("There are {} constellations.", constellations);

    Ok(())
}

fn parse_point(s: &str) -> Result<Vec<i32>, ParseIntError> {
    s.split(',').map(|n| n.trim().parse()).collect()
}

fn manhattan(a: &[i32], b: &[i32]) -> i32 {
    a.iter().zip(b.iter()).map(|(x, y)| i32::abs(x - y)).sum()
}

fn same_constellation(a: &[Vec<i32>], b: &[Vec<i32>]) -> bool {
    for star in a {
        if b.iter().any(|star_b| manhattan(star, star_b) <= 3) {
            return true;
        }
    }

    false
}

fn count_constellations<T: IntoIterator<Item = Vec<i32>>>(stars: T) -> usize {
    let mut constellations: Vec<Vec<Vec<i32>>> = Vec::new();

    for star in stars {
        let mut new_constellation = vec![star];
        for constellation in &mut constellations {
            if same_constellation(&new_constellation, constellation) {
                new_constellation.append(constellation);
            }
        }

        constellations.retain(|c| !c.is_empty());
        constellations.push(new_constellation);
    }

    constellations.len()
}

fn count_constellations_in_list(input: &str) -> Result<usize, ParseIntError> {
    let points = input
        .lines()
        .map(parse_point)
        .collect::<Result<Vec<Vec<i32>>, ParseIntError>>()?;
    let constellations = count_constellations(points);
    Ok(constellations)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let constellations = count_constellations_in_list(
            r"0,0,0,0
 3,0,0,0
 0,3,0,0
 0,0,3,0
 0,0,0,3
 0,0,0,6
 9,0,0,0
12,0,0,0",
        )
        .unwrap();

        assert_eq!(2, constellations);
    }

    #[test]
    fn example2() {
        let constellations = count_constellations_in_list(
            r"-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0",
        )
        .unwrap();

        assert_eq!(4, constellations);
    }

    #[test]
    fn example3() {
        let constellations = count_constellations_in_list(
            r"1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2",
        )
        .unwrap();

        assert_eq!(3, constellations);
    }

    #[test]
    fn example4() {
        let constellations = count_constellations_in_list(
            r"1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2",
        )
        .unwrap();

        assert_eq!(8, constellations);
    }
}
