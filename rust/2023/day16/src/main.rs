use std::{error::Error, collections::{HashMap, VecDeque, HashSet}};
use aochelpers::{get_daily_input, Coordinate, Direction};

enum TileDirection {
    NorthWestSouthEast,
    NorthEastSouthWest,
    NorthSouth,
    EastWest
}
enum Tile {
    Empty,
    Mirror(TileDirection),
    Splitter(TileDirection)
}
#[derive(Debug,Clone, Copy,Hash,PartialEq, Eq)]
struct Particle {
    heading: Direction,
    location: Coordinate<i32>
}
fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(16,2023)?;
    let arena: HashMap<Coordinate<i32>, Tile> = parse_data(&data);
    println!("{}", part1(&arena, Particle{heading: Direction::East, location: Coordinate { x: 0, y: 0 }}));
    println!("{}", part2(&arena));

    Ok(())
}

fn part2(arena: &HashMap<Coordinate<i32>, Tile>) -> usize {
    let mut max_score = 0;
    let boundary: Coordinate<i32> = Coordinate{ 
        x: arena.keys().map(|c| c.x).max().unwrap(),
        y: arena.keys().map(|c| c.y).max().unwrap()
    }; 

    for x in 0..=boundary.x {
        max_score = max_score.max(part1(arena, Particle{heading: Direction::South, location: Coordinate{x, y:0}}));
    }
    for x in 0..=boundary.x {
        max_score = max_score.max(part1(arena, Particle{heading: Direction::North, location: Coordinate{x, y:boundary.y}}));
    }
    for y in 0..=boundary.y {
        max_score = max_score.max(part1(arena, Particle{heading: Direction::East, location: Coordinate{x:0, y}}));
    }
    for y in 0..=boundary.y {
        max_score = max_score.max(part1(arena, Particle{heading: Direction::West, location: Coordinate{x:boundary.x, y}}));
    }
    max_score
}
fn parse_data(data: &str) -> HashMap<Coordinate<i32>, Tile> {
    let mut tile_map = HashMap::new();

    for (y, line) in data.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '\\' => {
                    tile_map.insert(Coordinate { x: x as i32, y: y as i32 }, Tile::Mirror(TileDirection::NorthWestSouthEast));
                },
                '/' => {
                    tile_map.insert(Coordinate { x: x as i32, y: y as i32 }, Tile::Mirror(TileDirection::NorthEastSouthWest));
                },
                '|' => {
                    tile_map.insert(Coordinate { x: x as i32, y: y as i32 }, Tile::Splitter(TileDirection::NorthSouth));
                },
                '-' => {
                    tile_map.insert(Coordinate { x: x as i32, y: y as i32 }, Tile::Splitter(TileDirection::EastWest));
                }
                _ => {}
            }
        }
    }
    tile_map
}

fn part1(arena: &HashMap<Coordinate<i32>, Tile>, starting_point: Particle) -> usize {
    let mut unvisited: VecDeque<Particle> = VecDeque::new();
    let mut visited = HashSet::new();
    let mut seen_state = HashSet::new();

    let boundary: Coordinate<i32> = Coordinate{ 
        x: arena.keys().map(|c| c.x).max().unwrap(),
        y: arena.keys().map(|c| c.y).max().unwrap()
    }; 

    unvisited.push_back(starting_point);
    while ! unvisited.is_empty() {
        let current_loc = unvisited.pop_front().unwrap();
        if current_loc.location.x < 0 || current_loc.location.x > boundary.x ||
        current_loc.location.y < 0 || current_loc.location.y > boundary.y ||
        seen_state.contains(&current_loc){
            continue;
        }
        visited.insert(current_loc.location);
        seen_state.insert(current_loc);
        match arena.get(&current_loc.location).unwrap_or(&Tile::Empty) {
            Tile::Empty => {unvisited.push_back(Particle{location: current_loc.location.neighbour(current_loc.heading), heading: current_loc.heading});},
            Tile::Mirror(mirror_direction) => {
                match mirror_direction {
                    TileDirection::NorthWestSouthEast => { // \
                        match current_loc.heading {
                            Direction::North => {unvisited.push_back(Particle{heading: Direction::West, location: current_loc.location.neighbour(Direction::West)});},
                            Direction::East => {unvisited.push_back(Particle{heading: Direction::South, location: current_loc.location.neighbour(Direction::South)});},
                            Direction::South => {unvisited.push_back(Particle{heading: Direction::East, location: current_loc.location.neighbour(Direction::East)});},
                            Direction::West => {unvisited.push_back(Particle{heading: Direction::North, location: current_loc.location.neighbour(Direction::North)});},
                        _ => unimplemented!()         
                        };
                    },
                    TileDirection::NorthEastSouthWest => { // /
                        match current_loc.heading {
                            Direction::North => {unvisited.push_back(Particle{heading: Direction::East, location: current_loc.location.neighbour(Direction::East)});},
                            Direction::East => {unvisited.push_back(Particle{heading: Direction::North, location: current_loc.location.neighbour(Direction::North)});},
                            Direction::South => {unvisited.push_back(Particle{heading: Direction::West, location: current_loc.location.neighbour(Direction::West)});},
                            Direction::West => {unvisited.push_back(Particle{heading: Direction::South, location: current_loc.location.neighbour(Direction::South)});},
                        _ => unimplemented!()
                        };
                    },
                    TileDirection::NorthSouth => unimplemented!(),
                    TileDirection::EastWest => unimplemented!(),
                }
            }
            Tile::Splitter(splitter_direction) => {
                match splitter_direction {
                    TileDirection::NorthSouth => {
                        match current_loc.heading {
                            Direction::East | Direction::West => {
                                unvisited.push_back(Particle{heading: Direction::North, location: current_loc.location.neighbour(Direction::North)});
                                unvisited.push_back(Particle{heading: Direction::South, location: current_loc.location.neighbour(Direction::South)});
                            },
                            _ => {unvisited.push_back(Particle{location: current_loc.location.neighbour(current_loc.heading), heading: current_loc.heading});}
                        }
                    }
                    TileDirection::EastWest => {
                        match current_loc.heading {
                            Direction::North | Direction::South => {
                                unvisited.push_back(Particle{heading: Direction::East, location: current_loc.location.neighbour(Direction::East)});
                                unvisited.push_back(Particle{heading: Direction::West, location: current_loc.location.neighbour(Direction::West)});

                            },
                            _ => {unvisited.push_back(Particle{location: current_loc.location.neighbour(current_loc.heading), heading: current_loc.heading
                            });}
                        }
                    },
                    _ => {unimplemented!();}
                }
            },
        }
    }
    visited.len()
}

#[cfg(test)]
mod tests {

    use super::*; 

    const DATA: &str = 
".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";

    #[test]
    fn test_part1() {
        let arena: HashMap<Coordinate<i32>, Tile> = parse_data(DATA);
        assert_eq!(part1(&arena, Particle{heading: Direction::East, location: Coordinate { x: 0, y: 0 }}), 46);
        assert_eq!(part2(&arena), 51)
    }
}