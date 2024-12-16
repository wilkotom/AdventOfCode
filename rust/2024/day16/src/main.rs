use std::{collections::{BinaryHeap, HashMap, HashSet}, error::Error};
use aochelpers::{get_daily_input, Coordinate, Direction, Grid, ScoredItem};

#[derive(Debug,Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Square {
    Wall,
    Start,
    End,
    Empty
}

#[derive(Debug, Clone,Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position {
    location: Coordinate<usize>,
    facing: Direction
}

fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(16,2024)?;
    let grid = parse_data(&data);
    let (part1, part2) = solve(&grid);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    Ok(())
}

fn flood_fill(grid: &Grid<Square>, start_location: Position) -> HashMap<Position, usize> {
    let mut visited = HashMap::new();
    let mut unvisited = BinaryHeap::new();
    unvisited.push(ScoredItem{cost: 0, item: start_location});
    while let Some(state) = unvisited.pop() {
        if visited.contains_key(&state.item) {
            continue;
        }
        visited.insert(state.item, state.cost);
        match state.item.facing {
            Direction::North | Direction::South => {
                unvisited.push(ScoredItem{cost: state.cost + 1000, item: Position{ location: state.item.location, facing:Direction::East}});
                unvisited.push(ScoredItem{cost: state.cost + 1000, item: Position{ location: state.item.location, facing:Direction::West}});
                },
            Direction::East | Direction::West => {
                unvisited.push(ScoredItem{cost: state.cost + 1000, item: Position{ location: state.item.location, facing:Direction::North}});
                unvisited.push(ScoredItem{cost: state.cost + 1000, item: Position{ location: state.item.location, facing:Direction::South}});
            },
            _=> unimplemented!()
        }
        match grid.get(&state.item.location.neighbour(state.item.facing)) {
            Some(Square::Empty) |  Some(Square::End) | Some(Square::Start) =>{
                unvisited.push(ScoredItem{cost: state.cost + 1, item: Position{ location: state.item.location.neighbour(state.item.facing), facing:state.item.facing}});
            }
            Some(Square::Wall)  => {
            }
            _ => unimplemented!()
        }
    }
    visited
}

fn solve(grid: &Grid<Square>) -> (usize,usize) {
    let start = grid.iter::<usize>().find(|(_,s)| *s == Square::Start).map(|(c,_)| c).expect("No starting point");
    let end = grid.iter::<usize>().find(|(_,s)| *s == Square::End).map(|(c,_)| c).expect("No ending point");
    let start_map = flood_fill(grid, Position{location: start, facing: Direction::East});
    let end_map = flood_fill(grid, Position{location: end, facing: Direction::South});
    let optimum_distance = *start_map.get(&Position{location: end, facing: Direction::North}).unwrap_or(&0);    
    let mut good_squares = HashSet::new();
    for (start_dir, end_dir) in [(Direction::North, Direction::South), (Direction::East, Direction::West), (Direction::South, Direction::North), (Direction::West, Direction::East)] {
        for square in grid.keys::<usize>() {
            if *start_map.get(&Position{location: square, facing: start_dir}).unwrap_or(&0) + *end_map.get(&Position{location: square, facing: end_dir}).unwrap_or(&0) == optimum_distance  {
                good_squares.insert(square);
            }
        }
    }
    (*start_map.get(&Position{location: end, facing: Direction::North}).unwrap_or(&0), good_squares.len())
}

fn parse_data(input: &str) -> Grid<Square> {
    let mut grid = Grid::new();
    for (y, line) in input.lines().enumerate() {
        for (x,c) in line.chars().enumerate() {
            grid.insert(Coordinate{x,y}, match c {
                '#' => Square::Wall,
                '.' => Square::Empty,
                'S' => Square::Start,
                'E' => Square::End,
                _ => unimplemented!()
            });
        }
    }

    grid
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST1: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const TEST2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    fn test_small_grid() {
        let grid = parse_data(TEST1);
        assert_eq!(solve(&grid), (7036,45));
    }

    #[test]
    fn test_larger_grid() {
        let grid = parse_data(TEST2);
        assert_eq!(solve(&grid), (11048, 64));
    }

}