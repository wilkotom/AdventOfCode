use std::{collections::{HashMap, HashSet}, error::Error};
use aochelpers::{get_daily_input, Coordinate, Direction, Grid};
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(12,2024)?;
    let garden = parse_data(&data);
    let start: Instant = Instant::now();
    println!("Part 1: {}", solve(&garden, false));
    println!("Part 2: {}", solve(&garden, true));
    println!("Elapsed: {}ms", start.elapsed().as_millis());
    Ok(())
}

fn parse_data(data: &str) -> Grid<char> {
    let mut grid = Grid::new();
    for (y, line) in data.lines().enumerate() {
        for (x,c) in line.chars().enumerate() {
            grid.insert(Coordinate{x, y}, c);
        }    
    }
    grid
}


fn solve(garden: &Grid<char>, part2: bool) -> usize {
    let mut result = 0;
    let mut visited: HashSet<Coordinate<usize>> = HashSet::new();
    for (loc, label) in garden.iter() {
        if visited.contains(&loc) {
            continue;
        }
        let mut poly = HashSet::new();
        poly.insert(loc);
        let mut unconsidered = Vec::new();
        loc.checked_neighbours().filter(|n| ! visited.contains(n)).for_each(|n| unconsidered.push(n));
        while let Some (neighbour) = unconsidered.pop() {
            if visited.contains(&neighbour) { continue;}
            if garden.get(&neighbour) == Some(label) {
                visited.insert(neighbour);
                poly.insert(neighbour);
                neighbour.checked_neighbours().filter(|n| ! visited.contains(n)).for_each(|n| unconsidered.push(n));
            }
        }
        result += if part2 {get_sides(&poly)} else {get_perimeter(&poly)} * poly.len();
    }
    result
}

fn get_perimeter( field: &HashSet<Coordinate<usize>>) -> usize {
    let mut total = 0;
    for square in field.iter() {
        total += 4 - square.checked_neighbours().filter(|n| field.contains(&n)).count();
    }
    total
}

fn get_sides( field: &HashSet<Coordinate<usize>>) -> usize {
    let mut total = 0;

    // Number of edges is the same as the number of corners. Count the corners.
    for square in field.iter() {
        // External corners
        let attached_count = square.checked_neighbours().filter(|n| field.contains(n)).count();
        total += match attached_count {
            0 => 4,
            1 => 2,
            2 => if (square.checked_neighbour(Direction::North).is_some() && field.contains(&square.neighbour(Direction::North)) &&
                        square.checked_neighbour(Direction::South).is_some() && field.contains(&square.neighbour(Direction::South))) ||
                        (square.checked_neighbour(Direction::East).is_some() && field.contains(&square.neighbour(Direction::East)) &&
                        square.checked_neighbour(Direction::West).is_some() && field.contains(&square.neighbour(Direction::West)))  { 0 } else {1},
            _ => 0,
        };
        // Inner corners. If the diagonal neighbour is missing but the vertical and horizontal ones are not.

        for (widdershins, middle, turnwise) in [
                    (Direction::North, Direction::NorthEast, Direction::East), 
                    (Direction::East, Direction::SouthEast, Direction::South), 
                    (Direction::South, Direction::SouthWest, Direction::West), 
                    (Direction::West, Direction::NorthWest, Direction::North)] { 
            if ( square.checked_neighbour(middle).is_none() || !field.contains(&square.neighbour(middle))) && 
                    square.checked_neighbour(widdershins).is_some() && field.contains(&square.neighbour(widdershins))  && 
                    square.checked_neighbour(turnwise).is_some() && field.contains(&square.neighbour(turnwise))  {
                total += 1;
            }
        }
    }
    total
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