use std::collections::HashSet;

#[derive(Copy,Clone,Debug)]
enum CompassDirection {
   North,
   South,
   East,
   West
}

#[derive(Copy,Clone,Debug, PartialEq, Eq, Hash)]
struct Coordinate {
    x: i32,
    y: i32
}
#[derive(Copy,Clone,Debug)]
struct Location {
    facing: CompassDirection,
    loc: Coordinate
}

fn main() {
    let directions = std::fs::read_to_string(String::from("./input.txt")).unwrap().split(", ").map(|x| String::from(x)).collect::<Vec<_>>();

    let mut location = Location{facing: CompassDirection::North, loc: Coordinate{x: 0, y: 0}};
    let mut visited_locations: HashSet<Coordinate> = HashSet::new();
    let mut first_unvisited: Option<Coordinate> = None;
    for direction in directions.iter() {
        location.facing = match &direction[..1] {
            "L" => {
                match &location.facing {
                    CompassDirection::North => CompassDirection::West,
                    CompassDirection::West  => CompassDirection::South,
                    CompassDirection::South => CompassDirection::East,
                    CompassDirection::East => CompassDirection::North
                }
            },
            "R" => {
                match &location.facing {
                    CompassDirection::North => CompassDirection::East,
                    CompassDirection::East  => CompassDirection::South,
                    CompassDirection::South => CompassDirection::West,
                    CompassDirection::West => CompassDirection::North
                }
            },
            _ => { location.facing}
        };
        let mut distance: i32 = direction[1..].parse::<i32>().unwrap();
        match location.facing {
            CompassDirection::North => {
                while distance > 0 {
                    location.loc.y += 1;
                    distance -= 1;
                    if visited_locations.contains(&location.loc) {
                        if first_unvisited == None {
                            first_unvisited = Some(location.loc);
                        }
                    } else {
                        visited_locations.insert(location.loc);
                    }
                }
            },
            CompassDirection::South => {
                while distance > 0 {
                    location.loc.y -= 1;
                    distance -= 1;
                    if visited_locations.contains(&location.loc) {
                        if first_unvisited == None {
                            first_unvisited = Some(location.loc);
                        }
                    }
                    else {
                        visited_locations.insert(location.loc);
                    }
                }
            },
            CompassDirection::East => {
                while distance > 0 {
                    location.loc.x += 1;
                    distance -= 1;
                    if visited_locations.contains(&location.loc) {
                        if first_unvisited == None {
                            first_unvisited = Some(location.loc);
                        }
                    } else {
                        visited_locations.insert(location.loc);
                    }
                }
            },
            CompassDirection::West => {
                while distance > 0 {
                    location.loc.x -= 1;
                    distance -= 1;
                    if visited_locations.contains(&location.loc) {
                        if first_unvisited == None {
                            first_unvisited = Some(location.loc);
                        }
                    } else {
                        visited_locations.insert(location.loc);
                    }
                }
            }
        }
    }
    println!("Part 1: {}", location.loc.x.abs() + location.loc.y.abs());
    let part_2_location = first_unvisited.unwrap_or(Coordinate{x:0, y:0});
    println!("Part 2: {}", part_2_location.x.abs() + part_2_location.y.abs());

}