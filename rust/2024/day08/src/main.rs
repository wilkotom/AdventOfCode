use std::{collections::{HashMap, HashSet}, error::Error};
use aochelpers::{get_daily_input, Coordinate};

fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(8,2024)?;
    let (parsed, bounds) = parse_data(&data);
    println!("Part 1: {}", part1(&parsed, &bounds, false));
    println!("Part 2: {}", part1(&parsed, &bounds, true));
    Ok(())
}

fn parse_data(data: &str) -> (HashMap<char,Vec<Coordinate<i32>>>, Coordinate<i32>) {
    let mut parsed: HashMap<char,Vec<Coordinate<i32>>> = HashMap::new();
    let mut max_boundary = Coordinate{x:0_i32, y:0};
    for (y,line) in data.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c{
                c if c.is_ascii_alphanumeric() => {parsed.entry(c).or_default().push(Coordinate{x: x as i32, y: y as i32});}
                '.' | '#' => {}
                _ => unimplemented!()
            }
            max_boundary.x = max_boundary.x.max(x as i32);
        }
        max_boundary.y = max_boundary.x.max(y as i32);
    }
    (parsed, max_boundary)
}

fn part1(beacons: &HashMap<char,Vec<Coordinate<i32>>>, bounds: &Coordinate<i32>, part2: bool) -> usize {
    let mut antinodes = HashSet::new();
    for beacon_list in beacons.values() {
        for (i,first) in beacon_list.iter().enumerate() {
            for second in beacon_list[i+1..].iter() {
                let delta = *second - *first;
                for antinode in [*first - delta, *second + delta] {
                    if (0..=bounds.x).contains(&antinode.x) &&(0..=bounds.y).contains(&antinode.y) {
                        antinodes.insert(antinode);
                    }
                }
                if part2 {
                    let mut antinode = *first;
                    while (0..=bounds.x).contains(&antinode.x) &&(0..=bounds.y).contains(&antinode.y) {
                        antinodes.insert(antinode);
                        antinode -= delta;
                    }
                    antinode = *second;
                    while (0..=bounds.x).contains(&antinode.x) &&(0..=bounds.y).contains(&antinode.y) {
                        antinodes.insert(antinode);
                        antinode += delta;
                    }
    
                }
            }
        }
    }
    antinodes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const P1EX1:&str = "..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
..........";

    const P1EX2: &str = "..........
..........
..........
....a.....
........a.
.....a....
..........
......A...
..........
..........";

    const P1EX3: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

const P2EX1: &str = "T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........";

    #[test]
    fn test_part1_ex1() {
        let (parsed, bounds) = parse_data(P1EX1);
        assert_eq!(part1(&parsed, &bounds, false),2);     
    }

    #[test]
    fn test_part1_ex2() {
        let (parsed, bounds) = parse_data(P1EX2);
        assert_eq!(part1(&parsed, &bounds, false),4);
    }

    #[test]
    fn test_part1_ex3() {
        let (parsed, bounds) = parse_data(P1EX3);
        assert_eq!(part1(&parsed, &bounds, false),14);
    }

    #[test]
    fn test_part2_ex1() {
        let (parsed, bounds) = parse_data(P2EX1);
        assert_eq!(part1(&parsed, &bounds, true),9);
    }

    #[test]
    fn test_part2_ex2() {
        let (parsed, bounds) = parse_data(P1EX3);
        assert_eq!(part1(&parsed, &bounds, true),34);
    }
}