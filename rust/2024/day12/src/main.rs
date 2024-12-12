use std::{collections::{HashMap, HashSet}, error::Error};
use aochelpers::{get_daily_input, Coordinate, Direction, Grid};

fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(12,2024)?;
    let garden = parse_data(&data);
    println!("Part 1: {}", solve(&garden, false));
    println!("Part 2: {}", solve(&garden, true));

    Ok(())
}

fn solve(garden: &HashMap<Coordinate<i32>,char>, part2: bool) -> usize {
    let mut result = 0;
    let mut visited = HashSet::new();
    for (loc, label) in garden.iter() {
        if visited.contains(loc) {
            continue;
        }
        let mut poly = HashSet::new();
        poly.insert(*loc);
        let mut unconsidered = Vec::new();
        loc.neighbours().into_iter().filter(|n| ! visited.contains(n)).for_each(|n| unconsidered.push(n));
        while let Some (neighbour) = unconsidered.pop() {
            if visited.contains(&neighbour) { continue;}
            if garden.get(&neighbour) == Some(label) {
                visited.insert(neighbour);
                poly.insert(neighbour);
                neighbour.neighbours().iter().filter(|n| ! visited.contains(n)).for_each(|n| unconsidered.push(*n));
            }
        }
        result += if part2 {get_sides(&poly)} else {get_perimeter(&poly)} * poly.len();
    }
    result
}

fn get_perimeter( field: &HashSet<Coordinate<i32>>) -> usize {
    let mut total = 0;
    for square in field.iter() {
        total += square.neighbours().into_iter().filter(|n| !field.contains(n)).count();
    }
    total
}

fn get_sides( field: &HashSet<Coordinate<i32>>) -> usize {
    let mut total = 0;

    // Number of edges is the same as the number of corners. Count the corners.
    for square in field.iter() {
        // External corners
        let unattached_count = square.neighbours().into_iter().filter(|n| !field.contains(n)).count();
        total += match unattached_count {
            4 => 4,
            3 => 2,
            2 => if (field.contains(&square.neighbour(Direction::North)) && field.contains(&square.neighbour(Direction::South))) ||
                    (field.contains(&square.neighbour(Direction::East)) && field.contains(&square.neighbour(Direction::West))) { 0 } else {1},
            _ => 0,
        };

        // Inner corners. If the diagonal neighbour is missing but the vertical and horizontal ones are not.

        if !field.contains(&square.neighbour(Direction::NorthEast)) && 
                field.contains(&square.neighbour(Direction::North))  && 
                field.contains(&square.neighbour(Direction::East)) {
            total += 1;
        }
        if !field.contains(&square.neighbour(Direction::SouthEast)) && 
                field.contains(&square.neighbour(Direction::South))  && 
                field.contains(&square.neighbour(Direction::East)) {
            total += 1;
        }
        if !field.contains(&square.neighbour(Direction::NorthWest)) && 
                field.contains(&square.neighbour(Direction::North))  && 
                field.contains(&square.neighbour(Direction::West)) {
            total += 1;
        }
        if !field.contains(&square.neighbour(Direction::SouthWest)) && 
                field.contains(&square.neighbour(Direction::South))  && 
                field.contains(&square.neighbour(Direction::West)) {
            total += 1;
        }
    }
    total
}

fn parse_data(data: &str) -> HashMap<Coordinate<i32>, char> {
    let mut grid = HashMap::new();
    for (y, line) in data.lines().enumerate() {
        for (x,c) in line.chars().enumerate() {
            grid.insert(Coordinate{x: x as i32,y: y as i32}, c);
        }    
    }
    grid
}


#[cfg(test)]
mod tests {

    const P1EXAMPLE1: &str = "AAAA
BBCD
BBCC
EEEC";

    const P1EXAMPLE2: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
    use super::*;

    #[test]
    fn test_part1_example1() {
        let data = parse_data(P1EXAMPLE1);
        assert_eq!(solve(&data, false), 140);
    }

    #[test]
    fn test_part1_example2() {
        let data = parse_data(P1EXAMPLE2);
        assert_eq!(solve(&data, false), 1930);
    }

    #[test]
    fn test_part2() {
        let data = parse_data(P1EXAMPLE1);
        assert_eq!(solve(&data, true), 80);
    }

    #[test]
    fn test_part2_ex2() {
        let data = parse_data(P1EXAMPLE2);
        assert_eq!(solve(&data,true), 1206);
    }

    #[test]
    fn test_count_edges_1_by_3() {
        assert_eq!(
            get_sides(&HashSet::from([Coordinate { x: 0, y: 3 }, Coordinate { x: 2, y: 3 }, Coordinate { x: 1, y: 3 }])), 4)
    }

    #[test]
    fn test_count_edges_2_by_2() {
        assert_eq!(
            get_sides(&HashSet::from([Coordinate { x: 0, y: 0 }, Coordinate { x: 1, y: 0 }, Coordinate { x: 0, y: 1 }, Coordinate{x:1, y:1}])), 4)
    }
}