use std::{collections::HashMap, ops::{Add, AddAssign}};
#[derive(Debug,Clone, Copy, PartialEq, Eq)]
enum MapLocation {
    Track(Track),
    Letter(char),
    Empty
}

#[derive(Debug,Clone, Copy, Hash, PartialEq, Eq)]
struct Coordinate {
    x: i32,
    y: i32
}

impl Add for Coordinate{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Coordinate{x: self.x + other.x, y: self.y + other.y}
        
    }
}

impl AddAssign for Coordinate {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

#[derive(Debug,Clone, Copy)]
struct Packet {
    location: Coordinate,
    direction: Coordinate
}


#[derive(Debug,Clone, Copy, PartialEq, Eq)]
enum Track {
    NorthSouth,
    EastWest,
    Corner
}

fn main() {
    let data = std::fs::read_to_string("./test.txt").unwrap();

    let mut track_map: HashMap<Coordinate, MapLocation> = HashMap::new();
    let mut packet = Packet{location:Coordinate{x:0, y:0}, direction: Coordinate{x: 0, y:1}};

    for (y, line) in data.split('\n').enumerate() {
        for (x,c) in line.chars().enumerate() {
            match c {
                '|'  => {
                    track_map.insert(Coordinate{x: x as i32,y: y as i32}, MapLocation::Track(Track::NorthSouth));
                    if y == 0 {
                        packet.location.x = x as i32;
                    }
                },
                '-'  => {
                    track_map.insert(Coordinate{x: x as i32,y: y as i32}, MapLocation::Track(Track::EastWest));
                },
                '+'  => {
                    track_map.insert(Coordinate{x: x as i32,y: y as i32}, MapLocation::Track(Track::Corner));
                },
                ' ' => {},
                _ => {
                    track_map.insert(Coordinate{x: x as i32,y: y as i32}, MapLocation::Letter(c));
                }
            }
        }

    }
    let mut steps = 0;
    let mut output = String::new();
    loop {
        steps +=1;

        // println!("{:?}", track_map.get(&packet.location));
        match track_map.get(&packet.location) {
            Some(MapLocation::Track(_)) => {},
            Some(MapLocation::Letter(c)) => {output.push(*c)},
            _ => {}
        }
        if track_map.get(&(packet.location + packet.direction)).unwrap_or(&MapLocation::Empty) == &MapLocation::Empty {        
            packet.direction = Coordinate{x: -packet.direction.y, y: packet.direction.x};
            if track_map.get(&(packet.location + packet.direction)).unwrap_or(&MapLocation::Empty) == &MapLocation::Empty {
                packet.direction = Coordinate{x: -packet.direction.x, y: -packet.direction.y}
            }
            
        }
        if track_map.get(&(packet.location + packet.direction)).unwrap_or(&MapLocation::Empty) == &MapLocation::Empty {
            break;
        }
        packet.location += packet.direction;
    } 

    println!("{} {:?}", output, steps);
}
