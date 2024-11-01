use std::fs::read_to_string;
use hashbrown::{HashMap, HashSet};

#[derive(Debug,Clone,Copy, Hash, PartialEq, Eq)]
struct Location {
    x: i32,
    y: i32,
}

#[derive(Debug,Clone,Copy, Eq, Hash, PartialEq)]
struct Doors {
    north: bool,
    south: bool,
    east: bool,
    west: bool,
    distance: i32
}

fn main() {
    let directions = read_to_string("./input.txt").unwrap();
    let current_location = Location{x:0, y:0};
    let mut maze_map = HashMap::new();
    maze_map.insert(current_location, Doors{north: false, south: false, east: false, west: false, distance: 0});

    follow_route(&mut maze_map, &directions[1..directions.len()-1], current_location);

    print_map(&maze_map);
    println!("Max distance: {}", maze_map.values().map(|v| v.distance).max().unwrap());
    println!("Rooms over 1000 doors away:{}", maze_map.values().filter(|v| v.distance >=1000).count());

}

fn follow_route(maze_map: &mut HashMap<Location, Doors>, route: &str, start_point: Location) -> HashSet<Location> {
    let res = split_section(route);
    let mut starting_points = HashSet::new();
    starting_points.insert(start_point);

    for fragment in res {
        for location in starting_points.clone() {
            starting_points = handle_segment(maze_map, location, fragment);        
        }
    }
    starting_points
}

fn handle_segment(maze_map: &mut HashMap<Location, Doors>, start_point: Location, segment: &str ) -> HashSet<Location> {
    let mut location = start_point;
        let mut i = 0;
        while segment.chars().nth(i).is_some() && segment.chars().nth(i) != Some('(') {
            let distance = maze_map.get(&location).unwrap().distance +1 ;
            match segment.chars().nth(i) {
                Some('N') => {
                    maze_map.get_mut(&location).unwrap().north = true;
                    location.y +=1;
                    maze_map.entry(location).or_insert(Doors{north: false, south: false, east: false, west: false, distance});
                    maze_map.get_mut(&location).unwrap().south = true;
                }
                Some('S') => {
                    maze_map.get_mut(&location).unwrap().south = true;
                    location.y -=1;
                    maze_map.entry(location).or_insert(Doors{north: false, south: false, east: false, west: false, distance});
                    maze_map.get_mut(&location).unwrap().north = true;

                }
                Some('E') => {
                    maze_map.get_mut(&location).unwrap().east = true;
                    location.x +=1;
                    maze_map.entry(location).or_insert(Doors{north: false, south: false, east: false, west: false, distance});
                    maze_map.get_mut(&location).unwrap().west = true;

                }
                Some('W')=> {
                    maze_map.get_mut(&location).unwrap().west = true;
                    location.x -=1;
                    maze_map.entry(location).or_insert(Doors{north: false, south: false, east: false, west: false, distance});
                    maze_map.get_mut(&location).unwrap().east = true;

                }
                _ => unimplemented!()

            }
            i+=1;
        }
        // Now need to handle branching paths
        let mut endpoints = HashSet::new();
        if segment.len() > i +1 {
            for segment in split_by_bar(&segment[i+1..segment.len()-1]) {
                endpoints = follow_route(maze_map, segment, location);
            }
        } else {
            endpoints.insert(location);
        }

    endpoints
}

fn print_map(maze_map: &HashMap<Location, Doors>) {
    let mut max_y = 0;
    let mut min_y = 0;
    let mut max_x =0;
    let mut min_x =0;
    for loc in maze_map.keys() {
        max_x = loc.x.max(max_x);
        min_x = loc.x.min(min_x);
        max_y = loc.y.max(max_y);
        min_y = loc.y.min(min_y);
    }
    for y in (min_y..max_y+1).rev() {
        for x in min_x..max_x+1 {
            print!("{}", match maze_map.get(&Location{x,y}).unwrap_or(&Doors{north: false, south: false, east: false, west: false, distance: 0}) {
                Doors{north: true, south: false, east: false, west: false, distance: _} => { '╵' },
                Doors{north: false, south: true, east: false, west: false, distance: _} => { '╷' },
                Doors{north: false, south: false, east: true, west: false, distance: _} => { '╶' },
                Doors{north: false, south: false, east: false, west: true, distance: _} => { '╴' },
                Doors{north: true, south: true, east: false, west: false, distance: _} => { '│' },
                Doors{north: false, south: false, east: true, west: true, distance: _} => { '─' },
                Doors{north: true, south: false, east: true, west: false, distance: _} => { '└' },
                Doors{north: true, south: false, east: false, west: true, distance: _} => { '┘' },
                Doors{north: false, south: true, east: true, west: false, distance: _} => { '┌' },
                Doors{north: false, south: true, east: false, west: true, distance: _} => { '┐' },
                Doors{north: false, south: true, east: true, west: true, distance: _} => { '┬' },
                Doors{north: true, south: true, east: false, west: true, distance: _} => { '┤' },
                Doors{north: true, south: false, east: true, west: true, distance: _} => { '┴' },
                Doors{north: true, south: true, east: true, west: false, distance: _} => { '├' },
                Doors{north: true, south: true, east: true, west: true, distance: _} => { '┼' },
                _ => ' '
            });
        }
        println!();
    }
}
 
fn split_section (mut directions: &str) -> Vec<&str> {
    let mut i = 0;
    let mut splits = Vec::new();
    let mut boundary = false;
    while !directions.is_empty() && directions.contains('('){
        
        while let Some(c) = directions.chars().nth(i) {
            if c == '(' {
                let mut level = 1;
                while level > 0 {
                    i +=1;
                    match directions.chars().nth(i) {
                        Some('(') => level +=1,
                        Some(')') => level -=1,
                        _ => {}
                    }
                }
                splits.push(&directions[0..i+1]);
                directions =  &directions[i+1..];
                boundary = true;
            };
            if boundary {
                i = 0;
                boundary = false;
            } else {
                i +=1;
            }
        }
    }
    if ! directions.is_empty() {
        splits.push(directions);
    }
    splits
}

fn split_by_bar(section: &str) -> Vec<&str> {
    let mut i = 0;
    let mut lower = 0;
    let mut sections = Vec::new();
    while let Some(c) =  section.chars().nth(i) {
        match c {
            '|' => { sections.push(&section[lower..i]);
                lower = i+1;
            },
            '(' => {
                let mut level = 1;
                while level > 0 {
                    i +=1;
                    match section.chars().nth(i) {
                        Some('(') => level +=1,
                        Some(')') => level -=1,
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        i +=1;
    }
    sections.push(&section[lower..]);
    sections
}
