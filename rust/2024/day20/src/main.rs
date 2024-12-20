use std::{collections::{BinaryHeap, HashMap, HashSet}, error::Error};
use aochelpers::{get_daily_input, Coordinate, Grid, ScoredItem};

#[derive(Debug, Copy,Clone, PartialEq,Eq)]
enum Square {
    Start,
    Finish,
    Wall,
    Empty
}

#[derive(Debug, Copy,Clone, PartialEq,Eq, Ord, PartialOrd)] 
struct Position {
    steps_taken: i32,
    position: Coordinate<i32>,
    cheats_used: i32
}


fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(20,2024)?;
    let grid = parse_data(&data);
    println!("Part 1: {}", solve(&grid, 100, 2));
    println!("Part 2: {}", solve(&grid, 100, 20));

    Ok(())
}

fn solve(grid: &HashMap<Coordinate<i32>, Square>, target_saving: i32, max_cheat_duration: i32) -> i32{
    let start = grid.iter().find(|(_,v)| **v == Square::Start).map(|(c,_)|c).expect("No start Found");
    let end = grid.iter().find(|(_,v)| **v == Square::Finish).map(|(c,_)|c).expect("No start Found");

    let max_x = grid.keys().map(| c| c.x).max().unwrap();
    let max_y = grid.keys().map(| c| c.y).max().unwrap();

    let mut count = 0;
    let times_from_start= get_times(grid, start, end);
    let times_from_end= get_times(grid, end, start);
    let worst_time = times_from_start.get(end).unwrap();
    for square in times_from_start.keys() {
        for target in filled_manhattan_circle(square, max_cheat_duration) {
            if target.x <= 0 || target.x > max_x || target.y <= 0 || target.y > max_y || grid.get(&target) == Some(&Square::Wall) {
                continue;
            }
            if times_from_start.get(square).unwrap() + times_from_end.get(&target).unwrap() + square.manhattan_distance(&target) <= worst_time - target_saving {
                count +=1;
            }
        }
    }

    count
}

fn filled_manhattan_circle(start: &Coordinate<i32>, diameter: i32) -> HashSet<Coordinate<i32>> {
    let mut points = HashSet::new();
    let mut unvisted = Vec::new();
    unvisted.push(*start);
    while let Some(point) = unvisted.pop() {
        if point.manhattan_distance(start) > diameter {
            continue;
        }
        points.insert(point);
        for neighbour in point.neighbours() {
            if !points.contains(&neighbour){
                unvisted.push(neighbour);
            }
        }
    }

    points
}

fn get_times(grid: &HashMap<Coordinate<i32>, Square>, start: &Coordinate<i32>, end: &Coordinate<i32>) -> HashMap<Coordinate<i32>,i32> {
    let mut unvisited = BinaryHeap::new();
    let mut visited = HashMap::new();
    let initial_state = ScoredItem{cost: start.manhattan_distance(&end), item: Position{steps_taken: 0, position: *start, cheats_used: 0}};
    unvisited.push(initial_state);
    while let Some(state) = unvisited.pop() {
        if visited.contains_key(&state.item.position) {
            continue;
        }
        visited.insert(state.item.position, state.item.steps_taken);
        for neighbour in state.item.position.neighbours() {
            if grid.get(&neighbour) != Some(&Square::Wall) && !visited.contains_key(&neighbour) {
                unvisited.push(ScoredItem{cost: neighbour.manhattan_distance(end) + state.item.steps_taken +1, item: Position{position: neighbour, steps_taken: state.item.steps_taken +1, cheats_used:0} });
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
                c => {println!("Encountered unknwn square: {}", c);
                unimplemented!()}
            });
        }
    }
    grid
}


#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = "
###############
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
    fn test_base_case() {
        let grid = parse_data(TESTDATA);
        let start = grid.iter().find(|(_,v)| **v == Square::Start).map(|(c,_)|c).expect("No start Found");
        let end = grid.iter().find(|(_,v)| **v == Square::Finish).map(|(c,_)|c).expect("No start Found");
        let times= get_times(&grid, start, end);
        assert_eq!(times.get(end), Some(&84));
    }

    #[test]
    fn test_part1() {
        let grid: HashMap<Coordinate<i32>, Square>= parse_data(TESTDATA);
        assert_eq!(solve(&grid, 64, 2), 1); // There is cheats that save at least 64 ps
    }


    #[test]
    fn test_circle() {
        let points: HashSet<Coordinate<i32>> = filled_manhattan_circle(&Coordinate { x: 1, y: 1 },1);
        assert_eq!(points, HashSet::from([Coordinate{x:1, y:1}, Coordinate{x:1, y:0}, Coordinate{x:0, y:1}, Coordinate{x:2, y:1}, Coordinate{x:1, y:2}]));

    } 
    #[test]
    fn test_part2() {
        let grid: HashMap<Coordinate<i32>, Square>= parse_data(TESTDATA);
        assert_eq!(solve(&grid, 74, 20), 7); // There are 3 cheats that save at least 74 ps


    }
}