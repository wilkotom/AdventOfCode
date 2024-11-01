use std::{error::Error, collections::{HashMap, HashSet, VecDeque}};
use aochelpers::{get_daily_input, Direction, Coordinate};

type DistanceMap = HashMap<Coordinate<usize>, HashMap<Coordinate<usize>, usize>>;

#[derive(Debug,Clone, Copy, PartialEq, Eq)]
enum SquareType{
    Start,
    Goal,
    OneWay(Direction),
    Empty,
}


fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(23,2023)?;
    let arena: HashMap<Coordinate<usize>, SquareType> = parse_data(&data);

    println!("{:?}", solution(&arena, true));
    println!("{:?}", solution(&arena, false ));

    Ok(())
}

fn parse_data(data: &str) -> HashMap<Coordinate<usize>, SquareType> {
    let mut arena = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;
    for (y, line) in data.lines().enumerate() {
        for (x,c) in line.chars().enumerate() {
            match c {
                '.' => {arena.insert(Coordinate { x, y }, SquareType::Empty);}
                '^' => {arena.insert(Coordinate { x, y}, SquareType::OneWay(Direction::North));}
                '>' => {arena.insert(Coordinate { x, y}, SquareType::OneWay(Direction::East));}
                'v' => {arena.insert(Coordinate { x, y}, SquareType::OneWay(Direction::South));}
                '<' => {arena.insert(Coordinate { x, y}, SquareType::OneWay(Direction::West));}
                '#' => {}
                _ => unimplemented!()
            };
            max_x = x;
            max_y = y;
        }
    }
    arena.insert(Coordinate{x:1, y:0}, SquareType::Start);
    arena.insert(Coordinate{x:max_x-1, y:max_y}, SquareType::Goal);

    arena
}

fn solution(arena: &HashMap<Coordinate<usize>, SquareType>, part1: bool) -> usize {
    let junction_distances = generate_junction_distances(arena, part1);

    let mut paths = VecDeque::new();
    let starting_path = vec![Coordinate{x:1, y:0}];
    let finish_y = junction_distances.keys().map(|c| c.y).max().unwrap();
    let mut max_distance = 0;
    let mut complete_paths = Vec::new();
    paths.push_back(starting_path);
    while let Some(path) = paths.pop_front() {
        let last_value = path.iter().last().unwrap();
        if last_value.y == finish_y {
            complete_paths.push(path);
        } else {
            for next in junction_distances.get(last_value).unwrap().keys() {
                if ! path.contains(next) {
                    let mut next_path = path.clone();
                    next_path.push(*next);
                    paths.push_back(next_path);
                }
            }
        }

    }
    for path in complete_paths {

        let mut score = 0;
        for (i, junction) in path[..path.len()-1].iter().enumerate() {
            score += junction_distances[junction].get(&path[i+1]).unwrap();
        }
        max_distance = max_distance.max(score);
    }
    max_distance
}

fn generate_junction_distances(arena: &HashMap<Coordinate<usize>, SquareType>, part1: bool) ->  HashMap<Coordinate<usize>, 
HashMap<Coordinate<usize>, usize>> {
    // find all junction points ( or Goals)
    let mut junctions: DistanceMap = HashMap::new();
    for (square, squaretype) in arena.iter() {
        match squaretype {
            SquareType::Start | SquareType::Goal => {junctions.insert(*square, HashMap::new());},
            SquareType::Empty => {
                if square.neighbours().iter().filter(|n| arena.contains_key(n)).count() > 2 {
                    junctions.insert(*square, HashMap::new());
                }
            }
            SquareType::OneWay(_) => {},
        }
    }
    let junctions_vec = junctions.keys().cloned().collect::<Vec<_>>();
    for junction in junctions_vec.iter() {
        let mut visited: HashSet<Coordinate<usize>> = HashSet::new();
        let mut next_squares = VecDeque::new();
        next_squares.push_back((0, *junction));
        while let Some((distance, square)) = next_squares.pop_front() {

            if junctions.contains_key(&square) && square != *junction{
                let targets = junctions.get_mut(junction).unwrap();
                targets.insert(square, distance);
            }
            else {
                let square_type: Option<&SquareType> = arena.get(&square);

                match square_type {
                    Some(SquareType::Empty) => {
                        if visited.contains(&square) {
                            continue;
                        }
                        visited.insert(square);
                        square.neighbours().iter().for_each(|s| next_squares.push_back((distance+1, *s)))
                    },
                    Some(SquareType::OneWay(direction)) => {
                        if part1 {
                            next_squares.push_back((distance+1, square.neighbour(*direction)));
                        } else {
                            if visited.contains(&square) {
                                continue;
                            }
                            visited.insert(square);
                            square.neighbours().iter().for_each(|s| next_squares.push_back((distance+1, *s)))
    
                        }
                    }
                    Some(SquareType::Start) => {
                        visited.insert(square);
                        next_squares.push_back((distance+1, square.neighbour(Direction::South)));
                    }
                    Some(SquareType::Goal) | None => {}
                };
                
            }
        }

    }
junctions
}


#[cfg(test)]
mod tests {

    use super::*; 
    const DATA: &str = 
"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[test]
    fn test_parser() {
        let arena: HashMap<Coordinate<usize>, SquareType> = parse_data(DATA);
        assert_eq!(solution(&arena, true), 94) ;
        assert_eq!(solution(&arena, false), 154) ;

    }
}