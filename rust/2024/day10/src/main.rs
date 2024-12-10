use std::{collections::{HashMap, HashSet}, error::Error};
use aochelpers::{get_daily_input, parse_number_grid, Coordinate};

fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(10,2024)?;
    let grid = parse_number_grid(&data);
    println!("Part 1: {}", solve(&grid, false));
    println!("Part 2: {}", solve(&grid, true));

    Ok(())
}

fn solve(grid: &HashMap<Coordinate<isize>, u8>, part2: bool) -> usize {
    let mut result = 0;
    for trailhead in grid.iter().filter(|(_,v)| **v == 0).map(|(k,_)| k) {
        let mut visited = HashSet::new();
        let mut unvisited = Vec::new();
        unvisited.push(*trailhead);
        while let Some(next_loc) = unvisited.pop() {
            visited.insert(next_loc);
            if let Some(next_score) = grid.get(&next_loc) {
                if *next_score == 9 {
                    result +=1
                } else {
                    for neighbour in next_loc.neighbours() {
                        if let Some(neighbour_core) = grid.get(&neighbour) {
                            if *neighbour_core == next_score +1 && (!visited.contains(&neighbour) ||part2){
                                unvisited.push(neighbour);
                            }
                        }
                    }
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

const P1DATA: &str ="89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_part1() {
        let grid = parse_number_grid(P1DATA);
        assert_eq!(solve(&grid, false), 36);
    }

    #[test]
    fn test_part2() {
        let grid = parse_number_grid(P1DATA);
        assert_eq!(solve(&grid, true), 81);
    }
}