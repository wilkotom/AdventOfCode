use std::error::Error;
use aochelpers::{get_daily_input, Coordinate, Direction, Grid};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Square {
    Robot,
    Wall,
    Box,
    BoxLeft,
    BoxRight,
    Empty
}


fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(15,2024)?;
    let (grid, directions) = parse_data(&data);
    println!("Part 1: {}", part1(grid, &directions));
    let (grid, directions) = parse_data_p2(&data);
    println!("Part 2: {}", part2(grid, &directions));
    Ok(())
}

fn part1(mut grid: Grid<Square>, directions: &Vec<Direction>) -> usize {
    let mut robot: Coordinate<usize> = grid.iter().find(|(_, o)| o == &Square::Robot).map(|(c,_)| c).expect("There is no robot here");
    grid.insert(robot, Square::Empty);

    for direction in directions {
        let mut neighbour: Coordinate<usize> = robot.neighbour(*direction);
        match grid.get(&neighbour) {
            Some(Square::Wall) | Some(Square::Robot)=> {},
            Some(Square::Empty) => {
                grid.insert(neighbour, Square::Empty);
                robot = neighbour;
            },
            Some(Square::Box) => {
                while grid.get(&neighbour) == Some(Square::Box) {
                    neighbour = neighbour.neighbour(*direction);
                }
                match grid.get(&neighbour) {
                    Some(Square::Wall) | Some(Square::Robot) => {}
                    Some(Square::Empty) => {
                        grid.insert(neighbour, Square::Box);
                        grid.insert(robot.neighbour(*direction), Square::Empty);
                        robot = robot.neighbour(*direction);
                    }
                    None | Some(Square::Box) | Some(Square::BoxLeft) | Some(Square::BoxRight) => unimplemented!(),
                }
            }
            _ => unimplemented!(),
        }
    }
    grid.iter::<usize>().filter(|(_,s)| s == &Square::Box).map(|(c,_)| c.x + c.y*100).sum() 
}

fn part2(mut grid: Grid<Square>, directions: &Vec<Direction>) -> usize {
    let mut robot: Coordinate<usize> = grid.iter().find(|(_, o)| o == &Square::Robot).map(|(c,_)| c).expect("There is no robot here");
    grid.insert(robot, Square::Empty);

    for direction in directions {
        let mut neighbour = robot.neighbour(*direction);
        match grid.get(&neighbour) {
            Some(Square::Wall) | Some(Square::Robot)=> {},
            Some(Square::Empty) => {
                grid.insert(neighbour, Square::Empty);
                robot = neighbour;
            },
            Some(Square::BoxLeft) | Some(Square::BoxRight)=> {
                if can_move(neighbour, *direction, &grid) {
                    move_box(neighbour, *direction, &mut grid);
                    grid.insert(neighbour, Square::Empty);
                    robot = neighbour;    
                }
            }
            _ => unimplemented!(),
        }
    }
    grid.iter::<usize>().filter(|(_,s)| s == &Square::BoxLeft).map(|(c,_)| c.x + c.y*100).sum()
}

fn can_move(location: Coordinate<usize>, direction: Direction, grid: &Grid<Square>) -> bool {
    match direction {
        Direction::North => {
            match grid.get(&location) {
                Some(Square::BoxLeft) => {
                    match (grid.get(&Coordinate{x: location.x, y: location.y-1}), grid.get(&Coordinate{x: location.x+1, y: location.y-1})) {
                        (Some(Square::Empty), Some(Square::Empty)) => true,
                        (Some(Square::Wall), _) | (_, Some(Square::Wall)) => false,
                        (Some(Square::BoxLeft), Some(Square::BoxRight)) => can_move(Coordinate{x: location.x, y: location.y-1}, direction, grid),
                        (Some(Square::BoxRight), Some(Square::Empty)) => can_move(Coordinate{x: location.x-1, y: location.y-1}, direction, grid),
                        (Some(Square::Empty), Some(Square::BoxLeft)) => can_move(Coordinate{x: location.x+1, y: location.y-1}, direction, grid),
                        (Some(Square::BoxRight), Some(Square::BoxLeft)) => can_move(Coordinate{x: location.x-1, y: location.y-1}, direction, grid) && 
                                can_move(Coordinate{x: location.x+1, y: location.y-1}, direction, grid),
                        (_, _) => unimplemented!()
                    }
                }
                Some(Square::BoxRight) => {
                    can_move(Coordinate{x: location.x-1, y: location.y}, direction, grid)
                }
                _ => unimplemented!()
            }

        },
        Direction::East => {
            let target_location = Coordinate{x: location.x +2, y: location.y};
            match grid.get(&target_location) {
                Some(Square::Empty) => true,
                Some(Square::Wall) => false,
                Some(Square::BoxLeft) => can_move(target_location, direction, grid),
                None | Some(Square::Box)| Some(Square::BoxRight) | Some(Square::Robot) => unimplemented!(),
            }
        },
        Direction::South => {
            match grid.get(&location) {
                Some(Square::BoxLeft) => {
                    match (grid.get(&Coordinate{x: location.x, y: location.y+1}), grid.get(&Coordinate{x: location.x+1, y: location.y+1})) {
                        (Some(Square::Empty), Some(Square::Empty)) => true,
                        (Some(Square::Wall), _) | (_, Some(Square::Wall)) => false,
                        (Some(Square::BoxLeft), Some(Square::BoxRight)) => can_move(Coordinate{x: location.x, y: location.y+1}, direction, grid),
                        (Some(Square::BoxRight), Some(Square::Empty)) => can_move(Coordinate{x: location.x-1, y: location.y+1}, direction, grid),
                        (Some(Square::Empty), Some(Square::BoxLeft)) => can_move(Coordinate{x: location.x+1, y: location.y+1}, direction, grid),
                        (Some(Square::BoxRight), Some(Square::BoxLeft)) => can_move(Coordinate{x: location.x-1, y: location.y+1}, direction, grid) && 
                                can_move(Coordinate{x: location.x+1, y: location.y+1}, direction, grid),
                        (_, _) => unimplemented!()
                    }
                }
                Some(Square::BoxRight) => {
                    can_move(Coordinate{x: location.x-1, y: location.y}, direction, grid)
                }
                _ => unimplemented!()
            }
        },
        Direction::West => {
            let target_location: Coordinate<usize> = Coordinate{x: location.x -2, y: location.y};
            match grid.get(&target_location) {
                Some(Square::Empty) => true,
                Some(Square::Wall) => false,
                Some(Square::BoxRight) => can_move(target_location, direction, grid),
                _ => unimplemented!(),
            }
        },
        _ => unimplemented!()
    }
}

fn move_box(location: Coordinate<usize>, direction: Direction, grid: &mut Grid<Square>) {
    match grid.get(&location) {
        Some(Square::BoxLeft) => {
            match direction {
                Direction::North => {
                    match (grid.get(&Coordinate{x: location.x, y: location.y-1}), grid.get(&Coordinate{x: location.x+1, y: location.y-1})) {
                        (Some(Square::Empty), Some(Square::Empty)) => {
                        },
                        (Some(Square::BoxLeft), Some(Square::BoxRight)) => {
                            move_box(Coordinate{x: location.x, y: location.y-1}, direction, grid);
                        },
                        (Some(Square::BoxRight), Some(Square::Empty)) => {
                            move_box(Coordinate{x: location.x-1, y: location.y-1}, direction, grid);
                        },
                        (Some(Square::Empty), Some(Square::BoxLeft)) => {
                            move_box(Coordinate{x: location.x+1, y: location.y-1}, direction, grid);
                        },
                        (Some(Square::BoxRight), Some(Square::BoxLeft)) => {
                            move_box(Coordinate{x: location.x+1, y: location.y-1}, direction, grid);
                            move_box(Coordinate{x: location.x-1, y: location.y-1}, direction, grid);
                        },
                        (_, _) => unimplemented!()
                    };
                    grid.insert(Coordinate{x: location.x, y: location.y-1}, Square::BoxLeft);
                    grid.insert(Coordinate{x: location.x+1, y: location.y-1}, Square::BoxRight);
                    grid.insert(location, Square::Empty);
                    grid.insert(Coordinate{x: location.x+1, y: location.y}, Square::Empty);
                },
                
                Direction::South => {
                    match (grid.get(&Coordinate{x: location.x, y: location.y+1}), grid.get(&Coordinate{x: location.x+1, y: location.y+1})) {
                        (Some(Square::Empty), Some(Square::Empty)) => {},
                        (Some(Square::BoxLeft), Some(Square::BoxRight)) => {
                            move_box(Coordinate{x: location.x, y: location.y+1}, direction, grid);

                        },
                        (Some(Square::BoxRight), Some(Square::Empty)) => {
                            move_box(Coordinate{x: location.x-1, y: location.y+1}, direction, grid);
                        },
                        (Some(Square::Empty), Some(Square::BoxLeft)) => {
                            move_box(Coordinate{x: location.x+1, y: location.y+1}, direction, grid);
                        },
                        (Some(Square::BoxRight), Some(Square::BoxLeft)) => {
                            move_box(Coordinate{x: location.x+1, y: location.y+1}, direction, grid);
                            move_box(Coordinate{x: location.x-1, y: location.y+1}, direction, grid);
                        },
                        (_, _) => unimplemented!()
                    };
                    grid.insert(Coordinate{x: location.x, y: location.y+1}, Square::BoxLeft);
                    grid.insert(Coordinate{x: location.x+1, y: location.y+1}, Square::BoxRight);
                    grid.insert(location, Square::Empty);
                    grid.insert(Coordinate{x: location.x+1, y: location.y}, Square::Empty);
                }
                Direction::East => {
                    match grid.get(&Coordinate { x: location.x+2, y: location.y}) {
                        Some(Square::Empty) => { },
                        Some(Square::BoxLeft)  => {
                            move_box(Coordinate { x: location.x+2, y: location.y}, direction, grid);
                        }
                        _ => unimplemented!()
                    };
                    grid.insert(Coordinate { x: location.x+2, y: location.y}, Square::BoxRight);
                    grid.insert(Coordinate { x: location.x+1, y: location.y}, Square::BoxLeft);
                    grid.insert(location, Square::Empty);
                }
                Direction::West => {
                    match grid.get(&Coordinate { x: location.x-1, y: location.y}) {
                        Some(Square::Empty) => {},
                        Some(Square::BoxRight)  => {
                            move_box(Coordinate { x: location.x-1, y: location.y}, direction, grid);
                        }
                        _ => unimplemented!()
                    };
                    grid.insert(Coordinate { x: location.x-1, y: location.y}, Square::BoxLeft);
                    grid.insert( location, Square::BoxRight);
                    grid.insert(Coordinate { x: location.x+1, y: location.y}, Square::Empty);
                }
                _ => unimplemented!()
            }
        }
        Some(Square::BoxRight) => move_box(Coordinate { x: location.x -1, y: location.y }, direction, grid),
        _ => unimplemented!()
    }
}


fn parse_data(data: &str) -> (Grid<Square>, Vec<Direction>) {
    let mut sections = data.split("\n\n");
    let mut grid = Grid::new();
    for (y, line) in sections.next().unwrap().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.insert(Coordinate{x,y}, match c {
                '#' => Square::Wall,
                '.' => Square::Empty,
                'O' => Square::Box,
                '@' => Square::Robot,
                _ => unimplemented!()
            } );
        }
    }
    let directions = sections.next().unwrap().chars().filter(|c| *c != '\n').map(|c| match c {
        '^' => Direction::North,
        '<' => Direction::West,
        '>' => Direction::East,
        'v' => Direction::South,
        _=> {
            println!("Can't understand direction '{}'", c);
            unimplemented!();
        }
    }).collect();
    (grid, directions)

}



fn parse_data_p2(data: &str) -> (Grid<Square>, Vec<Direction>) {
    let mut sections = data.split("\n\n");
    let mut grid = Grid::new();
    for (y, line) in sections.next().unwrap().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.insert(Coordinate{x: x * 2,y}, match c {
                '#' => Square::Wall,
                '.' => Square::Empty,
                'O' => Square::BoxLeft,
                '@' => Square::Robot,
                _ => unimplemented!()
            } );
            grid.insert(Coordinate{x: x*2 +1,y}, match c {
                '#' => Square::Wall,
                '.' => Square::Empty,
                'O' => Square::BoxRight,
                '@' => Square::Empty,
                _ => unimplemented!()
            } );
        }
    }
    let directions = sections.next().unwrap().chars().filter(|c| *c != '\n').map(|c| match c {
        '^' => Direction::North,
        '<' => Direction::West,
        '>' => Direction::East,
        'v' => Direction::South,
        _=> {
            println!("Can't understand direction '{}'", c);
            unimplemented!();
        }
    }).collect();
    (grid, directions)

}


#[cfg(test)]
mod tests {
    use super::*;
    const SMALLTEST: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    const BIGTEST: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn test_part1() {
        let (grid, directions) = parse_data(SMALLTEST);
        assert_eq!(part1(grid, &directions), 2028);
    }

    #[test]
    fn test_part2() {
        let (grid, directions) = parse_data_p2(BIGTEST);
        assert_eq!(part2(grid, &directions), 9021);
    }

    #[test]
    fn test_part1_big() {
        let (grid, directions) = parse_data(BIGTEST);
        assert_eq!(part1(grid, &directions), 10092);
    }
}