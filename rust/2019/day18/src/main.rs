use std::{fs::read_to_string, cmp::Ordering, collections::BinaryHeap};
use hashbrown::{HashSet, HashMap};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MazeSquare {
    Wall,
    Key(char),
    Door(char),
    Entrance,
    Corridor
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Coordinate {
    x: usize,
    y: usize
}

impl Coordinate {
    fn neighbours(&self) -> [Coordinate; 4]{
        [Coordinate{x: self.x -1, y: self.y},
        Coordinate{x: self.x +1, y: self.y},
        Coordinate{x: self.x, y: self.y -1},
        Coordinate{x: self.x, y: self.y +1}]
    }
}

#[derive(Debug, Clone)]
struct ReachableRequirements {
    steps: usize,
    keys_needed: HashSet<char>
}

fn main() {
    let ascii_map = read_to_string("./day18/input.txt").unwrap();
    let mut maze = read_maze(&ascii_map);
    println!("Part 1: {}", solve(&maze)); 
    part2_maze(&mut maze);
    println!("Part 2: {}", solve(&maze));
}

fn part2_maze(maze: &mut HashMap<Coordinate, MazeSquare>) {
    let entrance = maze.iter().filter(|(_, s)| *s == &MazeSquare::Entrance).map(|(c, _)| *c).next().unwrap();

    maze.insert(entrance, MazeSquare::Wall);
    for neighbour in entrance.neighbours() {
        maze.insert(neighbour, MazeSquare::Wall);
    }

    maze.insert(Coordinate{x: entrance.x -1, y: entrance.y -1 }, MazeSquare::Entrance);
    maze.insert(Coordinate{x: entrance.x -1, y: entrance.y +1 }, MazeSquare::Entrance);
    maze.insert(Coordinate{x: entrance.x +1, y: entrance.y -1 }, MazeSquare::Entrance);
    maze.insert(Coordinate{x: entrance.x +1, y: entrance.y +1 }, MazeSquare::Entrance);
}

fn read_maze(ascii_map: &str) -> HashMap<Coordinate, MazeSquare> {
    let mut maze_map = HashMap::new();
    for (y, line) in ascii_map.split('\n').enumerate() {
        for (x, c) in line.chars().enumerate() {
            maze_map.insert(Coordinate{x,y},match c {
                '#' => MazeSquare::Wall,
                '.' => MazeSquare::Corridor,
                '@' => MazeSquare::Entrance,
                k if ('a'..='z').contains(&k) => MazeSquare::Key(k),
                d if ('A'..='Z').contains(&d) => MazeSquare::Door(d.to_lowercase().next().unwrap()),
                _ => unimplemented!()
            });
        }
    }

    maze_map
}

fn solve(maze: &HashMap<Coordinate, MazeSquare>) -> usize{

    let mut desired_state = HashSet::new();
    let mut distances_from = HashMap::new();
    let mut keys_to_locations = HashMap::new();
    let mut robots = Vec::new();
    for (loc, square) in maze {
        if let MazeSquare::Key(c) = square {
            distances_from.insert(*loc, distances(maze, *loc));
            keys_to_locations.insert(*c, *loc);
            desired_state.insert(*c);
        } else if *square == MazeSquare::Entrance {
            distances_from.insert(*loc, distances(maze, *loc));
            robots.push(*loc);
        }
    }
    
    let mut cache: HashMap<String, usize> = HashMap::new();

    best_distance(String::new(), keys_to_locations.keys().copied().collect::<HashSet<_>>(), &mut cache, &distances_from, &keys_to_locations, robots)
    
}


fn best_distance (visited: String, 
    unvisited: HashSet<char>, 
    cache: &mut HashMap<String, usize>, 
    distances_from: &HashMap<Coordinate, HashMap<char,ReachableRequirements>>,
    keys_to_locations: &HashMap<char,Coordinate>,
    robot_locations: Vec<Coordinate>) -> usize{
    let mut cache_key = unvisited.iter().sorted().collect::<String>();
    cache_key.push(visited.chars().last().unwrap_or('.'));
    if cache.contains_key(&cache_key) {
        cache[&cache_key]
    } else if unvisited.is_empty() {
        0
    } else {
        let mut possibles = Vec::new();
        for (i, robot) in robot_locations.iter().enumerate() {
            for next_dest in unvisited.clone()  {
                if !distances_from[robot].contains_key(&next_dest) || !distances_from[robot][&next_dest].keys_needed.iter().all(|c| visited.contains(*c)) {
                    continue;
                }
                let mut next_unvisited = unvisited.clone();
                next_unvisited.remove(&next_dest);
                let mut next_visited = visited.clone();
                next_visited.push(next_dest);
                let mut next_robots = robot_locations.clone();
                next_robots[i] = keys_to_locations[&next_dest];
                let distance = distances_from[robot][&next_dest].steps +
                     best_distance(next_visited, next_unvisited, cache, distances_from, keys_to_locations, next_robots);
                possibles.push(distance);
            }

        }
    
    let answer = *possibles.iter().min().unwrap();
    cache.insert(cache_key, answer);
    answer
    }
}


fn distances(maze: &HashMap<Coordinate, MazeSquare>, start: Coordinate) -> HashMap<char, ReachableRequirements>{

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct CrawlState {
        steps: usize,
        location: Coordinate,
        keys_needed: HashSet<char>
    }
    
    impl Ord for CrawlState {
        fn cmp(&self, other: &Self) -> Ordering {
            other.steps.cmp(&self.steps)
        }
    }
    
    impl PartialOrd for CrawlState {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let starting_state = CrawlState{steps:0, location: start, keys_needed: HashSet::new()};

    let mut unvisited = BinaryHeap::new();
    unvisited.push(starting_state);
    let mut distances = HashMap::new();
    let mut lookups = HashMap::new();

    while ! unvisited.is_empty() {
        let state = unvisited.pop().unwrap();
        match maze.get(&state.location) {
            Some(MazeSquare::Corridor) | Some(MazeSquare::Entrance) if ! distances.contains_key(&state.location) => {
                distances.insert(state.location, ReachableRequirements{steps: state.steps, keys_needed: state.keys_needed.clone()});
                for neighbour in state.location.neighbours() {
                    if ! distances.contains_key(&neighbour) {
                        unvisited.push( CrawlState{steps: state.steps +1, location: neighbour, keys_needed: state.keys_needed.clone()  });
                    }
                }
            },
            Some(MazeSquare::Door(c)) if ! distances.contains_key(&state.location) => {
                let mut new_keys_needed = state.keys_needed;
                new_keys_needed.insert(*c);
                distances.insert(state.location, ReachableRequirements{steps: state.steps, keys_needed: new_keys_needed.clone()});
                for neighbour in state.location.neighbours() {
                    if ! distances.contains_key(&neighbour) {
                        unvisited.push( CrawlState{steps: state.steps +1, location: neighbour, keys_needed: new_keys_needed.clone()  });
                    }
                }
            },
            Some(MazeSquare::Key(c)) if ! distances.contains_key(&state.location) => {
                let mut new_keys_needed = state.keys_needed.clone();
                new_keys_needed.insert(*c);
                distances.insert(state.location, ReachableRequirements{steps: state.steps, keys_needed: state.keys_needed.clone()});
                lookups.insert(c, state.location);
                for neighbour in state.location.neighbours() {
                    if ! distances.contains_key(&neighbour) {
                        unvisited.push( CrawlState{steps: state.steps +1, location: neighbour, keys_needed: new_keys_needed.clone()  });
                    }
                }
            }

            Some(_) | None => {}
        }
    }
    let mut interesting = HashMap::new();
    for (loc, sq) in maze {
        if let MazeSquare::Key(c) = sq {
            if distances.contains_key(loc) {
                interesting.insert(*c, distances[loc].clone());
            }
        }
    }
    interesting
}


#[test]

fn smallest_example() {
    let input = "#########\n#b.A.@.a#\n#########";
    let maze = read_maze(input);
    assert_eq!(solve(&maze), 8);
}

#[test]
fn small_example() {
    let input = "########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################";
    let maze = read_maze(input);
    assert_eq!(solve(&maze), 86);
}

#[test]
fn example_3() {
    let input = "########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################";
    let maze = read_maze(input);
    assert_eq!(solve(&maze), 132);
}

#[test]

fn example_4() {
    let input = "#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################";
let maze = read_maze(input);
assert_eq!(solve(&maze), 136);
}

#[test]
fn example_5() {
    let input ="########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################";
    let maze = read_maze(input);
    assert_eq!(solve(&maze), 81);
}

#[test]
fn example_6() {
    let input = "#######
#a.#Cd#
##@#@##
#######
##@#@##
#cB#.b#
#######";
    let maze = read_maze(input);
    assert_eq!(solve(&maze), 8);
}

#[test]
fn example_7() {
    let input = "###############
#d.ABC.#.....a#
######@#@######
###############
######@#@######
#b.....#.....c#
###############";
    let maze = read_maze(input);
    assert_eq!(solve(&maze), 24);
}


#[test]
fn example_8() {
    let input = "#############
#DcBa.#.GhKl#
#.###@#@#I###
#e#d#####j#k#
###C#@#@###J#
#fEbA.#.FgHi#
#############";
    let maze = read_maze(input);
    assert_eq!(solve(&maze), 32);
}


#[test]
fn example_9() {
    let input = "#############
#g#f.D#..h#l#
#F###e#E###.#
#dCba@#@BcIJ#
#############
#nK.L@#@G...#
#M###N#H###.#
#o#m..#i#jk.#
#############";
    let maze = read_maze(input);
    assert_eq!(solve(&maze), 72);
}

