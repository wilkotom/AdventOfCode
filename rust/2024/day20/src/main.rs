use std::{collections::HashMap, error::Error};
use aochelpers::{get_daily_input, Coordinate};

#[derive(Debug, Copy,Clone, PartialEq,Eq)]
enum Square {
    Start,
    Finish,
    Wall,
    Empty
}

fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(20,2024)?;
    let grid: HashMap<Coordinate<i32>, Square> = parse_data(&data);
    let squares = get_times(&grid);
    println!("Part 1: {}", solve(&squares, 100, 2));
    println!("Part 2: {}", solve(&squares, 100, 20));

    Ok(())
}

fn solve(squares: &[Coordinate<i32>], target_saving: i32, max_cheat_duration: i32) -> i32{
    let mut result = 0;
    for (i, source) in squares.iter().enumerate() {
        for (j, target) in squares[i+1..].iter().enumerate() {
            let cheat_distance = target.manhattan_distance(source);
            let saved_distance = j as i32 +1 - cheat_distance;
            if cheat_distance <= max_cheat_duration && saved_distance >= target_saving{
                result +=1;
            }
        }
    }
    result
}

fn get_times(grid: &HashMap<Coordinate<i32>, Square>) -> Vec<Coordinate<i32>> {
    let mut visited = Vec::new();
    let start = grid.iter().find(|(_,v)| **v == Square::Start).map(|(c,_)|c).expect("No start Found");
    let end = grid.iter().find(|(_,v)| **v == Square::Finish).map(|(c,_)|c).expect("No start Found");
    let mut current_loc = *start;
    visited.push(*start);
    while current_loc != *end {
        for neighbour in current_loc.neighbours() {
            if grid.get(&neighbour) != Some(&Square::Wall) && !visited.contains(&neighbour) {
                current_loc = neighbour;
                visited.push(current_loc);
                break;
            }
        }
    }
    visited
}

fn parse_data(data: &str) -> HashMap<Coordinate<i32>, Square> {
    let mut grid = HashMap::new();
    for (y, line) in data.lines().enumerate() {
        for (x,c) in line.chars().enumerate() {
            grid.insert(Coordinate{x: x as i32,y: y as i32}, match c  {
                'S' => Square::Start,
                'E' => Square::Finish,
                '.' => Square::Empty,
                '#' => Square::Wall,
                _ => unimplemented!()
            });
        }
    }
    grid
}


#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";


    #[test]
    fn test_part1() {
        let grid: HashMap<Coordinate<i32>, Square>= parse_data(TESTDATA);
        let squares = get_times(&grid);

        assert_eq!(solve(&squares, 64, 2), 1); // There is cheats that save at least 64 ps
    }

    #[test]
    fn test_part2() {
        let grid: HashMap<Coordinate<i32>, Square>= parse_data(TESTDATA);
        let squares = get_times(&grid);
        assert_eq!(solve(&squares, 76, 20), 3); // There are 3 cheats that save at least 76 ps
    }
}