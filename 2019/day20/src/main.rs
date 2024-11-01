use std::{collections::{HashMap, HashSet, BinaryHeap}, fs::read_to_string, cmp::Ordering};
#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, PartialOrd, Ord)]
struct Coordinate {
    x: usize,
    y: usize
}

impl Coordinate {
    fn get_neighbours(&self) -> [Coordinate; 4] {
        [
            Coordinate{x: self.x-1, y: self.y},
            Coordinate{x: self.x+1, y: self.y},
            Coordinate{x: self.x, y: self.y+1},
            Coordinate{x: self.x, y: self.y-1},
        ]
    }
}

#[derive(Debug,PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
enum MazeSquare {
    Wall,
    Corridor,
    InnerPortal(String),
    OuterPortal(String),
    Letter(char),
}

fn main() {
    let maze = read_to_string("./day20/input.txt").unwrap();
    println!("Part 1: {}",part1(&maze));
    println!("Part 2: {}",part2(&maze));

}

fn part1(maze: &str) -> usize {
    #[derive(PartialEq, Eq)]
    struct PossibleNode {
        steps: usize,
        location: MazeSquare
    }

    impl Ord for PossibleNode {
        fn cmp(&self, other: &Self) -> Ordering {
            other.steps.cmp(&self.steps)
        }
    
    }

    impl PartialOrd for PossibleNode {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let distances = parse_maze(maze);
    let mut minimum_distances = HashMap::new();
    let mut unvisited = BinaryHeap::new();
    unvisited.push(PossibleNode{steps: 0,location: MazeSquare::OuterPortal("AA".to_owned())});

    while !unvisited.is_empty() {
        let current = unvisited.pop().unwrap();
        if current.location == MazeSquare::OuterPortal("ZZ".to_owned()) {
            return current.steps;
        }
        if !minimum_distances.contains_key(&current.location) {
            minimum_distances.insert(current.location.clone(), current.steps);
            for (next_loc, distance) in distances.get(&current.location).unwrap() {
                unvisited.push(PossibleNode{steps: distance + current.steps, location: next_loc.clone()});
            }
        }
    }

    0
}



fn part2(maze: &str) -> usize {
    #[derive(PartialEq, Eq, Debug)]
    struct PossibleNode {
        steps: usize,
        level: i32,
        location: MazeSquare
    }

    impl Ord for PossibleNode {
        fn cmp(&self, other: &Self) -> Ordering {
            other.steps.cmp(&self.steps)
        }
    
    }

    impl PartialOrd for PossibleNode {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let distances = parse_maze(maze);
    let mut minimum_distances = HashMap::new();
    let mut unvisited = BinaryHeap::new();
    unvisited.push(PossibleNode{steps: 0,location: MazeSquare::OuterPortal("AA".to_owned()), level: 0});

    while !unvisited.is_empty() {
        let current_node = unvisited.pop().unwrap();
        if current_node.location == MazeSquare::OuterPortal("ZZ".to_owned()) && current_node.level == 0 {
            return current_node.steps;
        }
        if minimum_distances.contains_key(&(current_node.location.clone(), current_node.level)) { 
            continue;
        }
        if let MazeSquare::OuterPortal(portal) = current_node.location.clone() {
            match &portal[..] {
                "AA" | "ZZ" => {
                    if current_node.level != 0 {
                        continue;
                    }
                },
                _ => {
                    if current_node.level < 1 {
                        continue;
                    }
                }
            }
        }
        minimum_distances.insert((current_node.location.clone(), current_node.level), current_node.steps);
        for (next_loc, distance) in distances.get(&current_node.location).unwrap() {

            match (current_node.location.clone(), next_loc) {
                (MazeSquare::InnerPortal(starting), MazeSquare::OuterPortal(ending)) => {
                    if &starting == ending {
                        unvisited.push(PossibleNode{steps: distance + current_node.steps, location: next_loc.clone(),  level: current_node.level +1});
                    } else {
                        unvisited.push(PossibleNode{steps: distance + current_node.steps, location: next_loc.clone(),  level: current_node.level});
                    }
                },
                (MazeSquare::OuterPortal(starting), MazeSquare::InnerPortal(ending)) => {
                    if &starting == ending {
                        unvisited.push(PossibleNode{steps: distance + current_node.steps, location: next_loc.clone(),  level: current_node.level -1});
                    } else {
                        unvisited.push(PossibleNode{steps: distance + current_node.steps, location: next_loc.clone(),  level: current_node.level});
                    }

                },
                (MazeSquare::InnerPortal(_), MazeSquare::InnerPortal(_))  | (MazeSquare::OuterPortal(_), MazeSquare::OuterPortal(_)) => {
                    unvisited.push(PossibleNode{steps: distance + current_node.steps, location: next_loc.clone(),  level: current_node.level});
                },
                _ => unimplemented!(),

            }
        }
    }

    0
}


fn parse_maze(maze: &str) -> HashMap<MazeSquare, HashMap<MazeSquare, usize>> {
    let mut maze_map = HashMap::new();
    let min_x = 2;
    let mut max_x = 2;
    let min_y = 2;
    let mut max_y = 2;
    for (y, line) in maze.split('\n').enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    maze_map.insert(Coordinate{x,y},MazeSquare::Corridor);
                },
                'A'..='Z' => {
                    maze_map.insert(Coordinate{x,y},MazeSquare::Letter(c));
                },
                '#' => {
                    max_x = max_x.max(x);
                    max_y = max_y.max(y);
                }
                _ => {}
            };
        }
    }
    let corridor_locations = maze_map.keys().filter(|c| maze_map.get(c).unwrap() == &MazeSquare::Corridor).cloned().collect::<Vec<_>>();
    for square in corridor_locations {
        if let MazeSquare::Letter(f) = maze_map.get(&Coordinate{x: square.x, y: square.y - 1}).unwrap_or(&MazeSquare::Wall) {
            if let MazeSquare::Letter(l) = maze_map.get(&Coordinate{x: square.x, y: square.y - 2}).unwrap_or(&MazeSquare::Wall) {
                let label = format!("{}{}", l, f);
                let new_square = if square.y == min_y {MazeSquare::OuterPortal(label)} else {MazeSquare::InnerPortal(label)};
                 maze_map.insert(Coordinate{x: square.x, y: square.y}, new_square);
            } else {
                unimplemented!();
            }
        }
        if let MazeSquare::Letter(f) = maze_map.get(&Coordinate{x: square.x, y: square.y + 1}).unwrap_or(&MazeSquare::Wall) {
            if let MazeSquare::Letter(l) = maze_map.get(&Coordinate{x: square.x, y: square.y + 2}).unwrap_or(&MazeSquare::Wall) {
                let label = format!("{}{}", f, l);
                let new_square = if square.y == max_y {MazeSquare::OuterPortal(label)} else {MazeSquare::InnerPortal(label)};
                maze_map.insert(Coordinate{x: square.x, y: square.y}, new_square);
            } else {
                unimplemented!();
            }
        }

        if let MazeSquare::Letter(f) = maze_map.get(&Coordinate{x: square.x+1, y: square.y }).unwrap_or(&MazeSquare::Wall) {
            if let MazeSquare::Letter(l) = maze_map.get(&Coordinate{x: square.x +2, y: square.y}).unwrap_or(&MazeSquare::Wall) {
                let label = format!("{}{}", f, l);
                let new_square = if square.x == max_x {MazeSquare::OuterPortal(label)} else {MazeSquare::InnerPortal(label)};
                maze_map.insert(Coordinate{x: square.x, y: square.y}, new_square);
            } else {
                unimplemented!();
            }
        }

        if let MazeSquare::Letter(f) =  maze_map.get(&Coordinate{x: square.x-1, y: square.y }).unwrap_or(&MazeSquare::Wall) {
            if let MazeSquare::Letter(l) = maze_map.get(&Coordinate{x: square.x -2, y: square.y}).unwrap_or(&MazeSquare::Wall) {
                let label = format!("{}{}", l,f);
                let new_square = if square.x == min_x {MazeSquare::OuterPortal(label)} else {MazeSquare::InnerPortal(label)};
                maze_map.insert(Coordinate{x: square.x, y: square.y}, new_square);
            } else {
                unimplemented!();
            }
        }

    }

    let mut distances = HashMap::new();
    for (location, contents)  in maze_map.iter() {
        match contents {
            MazeSquare::InnerPortal(label) => {
                let mut lookup_table = get_distances(*location, &maze_map);
                lookup_table.insert(MazeSquare::OuterPortal(label.to_owned()), 1);
                distances.insert(MazeSquare::InnerPortal(label.to_owned()), lookup_table);
            },
            MazeSquare::OuterPortal(label) => {
                let mut lookup_table = get_distances(*location, &maze_map);
                if label != "AA" && label != "ZZ" {
                    lookup_table.insert(MazeSquare::InnerPortal(label.to_owned()), 1);
                }
                distances.insert(MazeSquare::OuterPortal(label.to_owned()), lookup_table);
            },
            _ => {}
        }
    }
    distances
}

fn get_distances(starting_point: Coordinate, maze_map: &HashMap<Coordinate, MazeSquare>) -> HashMap<MazeSquare, usize> {
    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
    struct NextSquare {
        distance: usize,
        location: Coordinate
    }


    let mut unvisited = starting_point.get_neighbours().iter().map(|c| NextSquare{location: *c, distance: 1}).collect::<BinaryHeap<_>>();
    let mut distances = HashMap::new();
    let mut visited = HashSet::new();
    visited.insert(starting_point);

    while ! unvisited.is_empty() {
        let square = unvisited.pop().unwrap();
        if ! visited.contains(&square.location) {
            visited.insert(square.location);
            match maze_map.get(&square.location).unwrap_or(&MazeSquare::Wall) {
                MazeSquare::InnerPortal(l) => {distances.insert(MazeSquare::InnerPortal(l.clone()), square.distance);},
                MazeSquare::OuterPortal(l) => {distances.insert(MazeSquare::OuterPortal(l.clone()), square.distance);},
                MazeSquare::Corridor => {
                    unvisited.append(&mut square.location.get_neighbours().iter().map(|c| NextSquare{location: *c, distance: square.distance + 1}).collect::<BinaryHeap<_>>());
                }
                _ => {},
            }

        }
    }
    distances
}


#[test]
fn smallest_maze_distances() {
    let maze = read_to_string("./tests/test1.txt").unwrap();
    let parsed = parse_maze(&maze);
    assert_eq!(parsed.get(&MazeSquare::OuterPortal("AA".to_owned())).unwrap_or(&HashMap::new()).get(&MazeSquare::OuterPortal("ZZ".to_owned())), Some(&26));
    assert_eq!(parsed.get(&MazeSquare::OuterPortal("AA".to_owned())).unwrap_or(&HashMap::new()).get(&MazeSquare::OuterPortal("BC".to_owned())), None);
    assert_eq!(parsed.get(&MazeSquare::OuterPortal("AA".to_owned())).unwrap_or(&HashMap::new()).get(&MazeSquare::InnerPortal("BC".to_owned())), Some(&4));
    assert_eq!(parsed.get(&MazeSquare::OuterPortal("AA".to_owned())).unwrap_or(&HashMap::new()).get(&MazeSquare::OuterPortal("DE".to_owned())), None);
    assert_eq!(parsed.get(&MazeSquare::OuterPortal("AA".to_owned())).unwrap_or(&HashMap::new()).get(&MazeSquare::InnerPortal("DE".to_owned())), None);
    assert_eq!(parsed.get(&MazeSquare::OuterPortal("AA".to_owned())).unwrap_or(&HashMap::new()).get(&MazeSquare::OuterPortal("FG".to_owned())), None);
    assert_eq!(parsed.get(&MazeSquare::OuterPortal("AA".to_owned())).unwrap_or(&HashMap::new()).get(&MazeSquare::InnerPortal("FG".to_owned())), Some(&30));
    assert_eq!(parsed.get(&MazeSquare::OuterPortal("ZZ".to_owned())).unwrap_or(&HashMap::new()).get(&MazeSquare::OuterPortal("AA".to_owned())), Some(&26));

}

