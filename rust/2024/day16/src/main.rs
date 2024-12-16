use std::{collections::{BinaryHeap, HashMap, HashSet}, error::Error, os::macos::raw::stat};
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct PositionWithPathTaken {
    location: Coordinate<usize>,
    facing: Direction,
    visited: Vec<(usize,Position)>
}

fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(16,2024)?;
    let grid = parse_data(&data);
    let (part1, part2) = solve(&grid);
    println!("Part 1: {}", part1);
    println!("Part 2: {:?}", part2);
    Ok(())
}

fn solve(grid: &Grid<Square>) -> (usize,usize) {

    let mut visited = HashMap::new();
    let mut unvisited = BinaryHeap::new();
    let start_location = grid.iter::<usize>().filter(|(_,s)| *s == Square::Start).map(|(c,_)|c).next().expect("There is no Start line");
    unvisited.push(ScoredItem{cost: 0, item: PositionWithPathTaken{ location: start_location, facing: Direction::East, visited: Vec::new()}});
    let mut best_path_score = usize::MAX;
    let mut best_paths = HashMap::new();
    while let Some(mut state) = unvisited.pop() {
        let cacheable_state = Position{location: state.item.location, facing: state.item.facing};
        if visited.contains_key(&cacheable_state) {
            continue;
        }
        visited.insert(cacheable_state, state.cost);
        state.item.visited.push((state.cost, cacheable_state));
        match state.item.facing {
            Direction::North => {
                unvisited.push(ScoredItem{cost: state.cost + 1000, item: PositionWithPathTaken{ location: state.item.location, facing:Direction::East, visited: state.item.visited.clone()}});
                unvisited.push(ScoredItem{cost: state.cost + 1000, item: PositionWithPathTaken{ location: state.item.location, facing:Direction::West, visited: state.item.visited.clone()}});
                },
            Direction::East => {
                unvisited.push(ScoredItem{cost: state.cost + 1000, item: PositionWithPathTaken{ location: state.item.location, facing:Direction::North, visited: state.item.visited.clone()}});
                unvisited.push(ScoredItem{cost: state.cost + 1000, item: PositionWithPathTaken{ location: state.item.location, facing:Direction::South, visited: state.item.visited.clone()}});
            },
            Direction::South => {
                unvisited.push(ScoredItem{cost: state.cost + 1000, item: PositionWithPathTaken{ location: state.item.location, facing:Direction::West, visited: state.item.visited.clone()}});
                unvisited.push(ScoredItem{cost: state.cost + 1000, item: PositionWithPathTaken{ location: state.item.location, facing:Direction::East, visited: state.item.visited.clone()}});

            },
            Direction::West => {
                unvisited.push(ScoredItem{cost: state.cost + 1000, item: PositionWithPathTaken{ location: state.item.location, facing:Direction::North, visited: state.item.visited.clone()}});
                unvisited.push(ScoredItem{cost: state.cost + 1000, item: PositionWithPathTaken{ location: state.item.location, facing:Direction::South, visited: state.item.visited.clone()}});
            }
            _=> unimplemented!()
        }
        let next_square =  grid.get(&state.item.location.neighbour(state.item.facing));
        match next_square {
            Some(Square::Empty) | Some(Square::Start) =>{
                unvisited.push(ScoredItem{cost: state.cost + 1, item: PositionWithPathTaken{ location: state.item.location.neighbour(state.item.facing), facing:state.item.facing, visited: state.item.visited}});
            }
            Some(Square::End) => {
                best_path_score = best_path_score.min(state.cost +1);
                if state.cost +1 == best_path_score {
                    for (c,l) in state.item.visited {
                        best_paths.insert(l, c);
                    }
                }
            }
            Some(Square::Wall) => {
            }
            _ => unimplemented!()
        }

    }
    unvisited.clear();
    unvisited.push(ScoredItem{cost: 0, item: PositionWithPathTaken{ location: start_location, facing: Direction::East, visited: Vec::new()}});
    while let Some(mut state) = unvisited.pop() {
        if state.cost > best_path_score {
            continue;
        }
        let cacheable_state = Position{location: state.item.location, facing: state.item.facing};
        if let Some(best_score) = best_paths.get(&cacheable_state) {
            match best_score.cmp(&state.cost) {
                std::cmp::Ordering::Less => {
                    continue;
                },
                std::cmp::Ordering::Equal => {
                    for (c, p) in state.item.visited.iter() {
                        best_paths.insert(*p, *c);
                    }
                },
                std::cmp::Ordering::Greater => {unimplemented!()},
            }
        } else if let Some(possible_cost) = visited.get(&cacheable_state) {
            if *possible_cost < state.cost {
             continue;
            }
        }
        if !best_paths.contains_key(&cacheable_state) {
            visited.insert(cacheable_state, state.cost);
            state.item.visited.push((state.cost, cacheable_state));
        }
        match state.item.facing {
            Direction::North => {
                unvisited.push(ScoredItem{cost: state.cost + 1000, item: PositionWithPathTaken{ location: state.item.location, facing:Direction::East, visited: state.item.visited.clone()}});
                unvisited.push(ScoredItem{cost: state.cost + 1000, item: PositionWithPathTaken{ location: state.item.location, facing:Direction::West, visited: state.item.visited.clone()}});
                },
            Direction::East => {
                unvisited.push(ScoredItem{cost: state.cost + 1000, item: PositionWithPathTaken{ location: state.item.location, facing:Direction::North, visited: state.item.visited.clone()}});
                unvisited.push(ScoredItem{cost: state.cost + 1000, item: PositionWithPathTaken{ location: state.item.location, facing:Direction::South, visited: state.item.visited.clone()}});
            },
            Direction::South => {
                unvisited.push(ScoredItem{cost: state.cost + 1000, item: PositionWithPathTaken{ location: state.item.location, facing:Direction::West, visited: state.item.visited.clone()}});
                unvisited.push(ScoredItem{cost: state.cost + 1000, item: PositionWithPathTaken{ location: state.item.location, facing:Direction::East, visited: state.item.visited.clone()}});

            },
            Direction::West => {
                unvisited.push(ScoredItem{cost: state.cost + 1000, item: PositionWithPathTaken{ location: state.item.location, facing:Direction::North, visited: state.item.visited.clone()}});
                unvisited.push(ScoredItem{cost: state.cost + 1000, item: PositionWithPathTaken{ location: state.item.location, facing:Direction::South, visited: state.item.visited.clone()}});
            }
            _=> unimplemented!()
        }
        let next_square =  grid.get(&state.item.location.neighbour(state.item.facing));
        match next_square {
            Some(Square::Empty) | Some(Square::Start) =>{
                unvisited.push(ScoredItem{cost: state.cost + 1, item: PositionWithPathTaken{ location: state.item.location.neighbour(state.item.facing), facing:state.item.facing, visited: state.item.visited}});
            }
            Some(Square::End) | Some(Square::Wall) => {
            }
            _ => unimplemented!()
        }


    }
    (best_path_score, best_paths.keys().map(|p| p.location).collect::<HashSet<_>>().len() +1)
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