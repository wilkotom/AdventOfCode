use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;

#[derive(Debug,Clone,Copy)]
enum Facing {
    North,
    South,
    East,
    West
}

#[derive(Debug,Clone,Copy)]
enum NextTurn {
    Left,
    Straight,
    Right
}

#[derive(Debug, PartialEq, Eq)]
enum Track {
    NorthSouth,
    EastWest,
    NorthwestSoutheast,
    SouthwestNortheast,
    CrossRoads,
    Empty
}

#[derive(Debug,Copy,Clone,PartialEq, Eq, Hash)]
struct Coordinate {
    x: usize,
    y: usize
}

impl Ord for Coordinate {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.y, self.x).cmp(&(other.y, other.x))
    }
}

impl PartialOrd for Coordinate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


#[derive(Debug,Clone, Copy)]
struct Cart {
    facing: Facing,
    next_turn: NextTurn,
    location: Coordinate,
}

impl Cart{

    fn turn(&mut self) {
        self.facing = match (self.facing, &self.next_turn) {
            (Facing::North, NextTurn::Left) => Facing::West,
            (Facing::North, NextTurn::Straight) => Facing::North,
            (Facing::North, NextTurn::Right) => Facing::East,
            (Facing::South, NextTurn::Left) => Facing::East,
            (Facing::South, NextTurn::Straight) => Facing::South,
            (Facing::South, NextTurn::Right) => Facing::West,
            (Facing::East, NextTurn::Left) => Facing::North,
            (Facing::East, NextTurn::Straight) => Facing::East,
            (Facing::East, NextTurn::Right) => Facing::South,
            (Facing::West, NextTurn::Left) => Facing::South,
            (Facing::West, NextTurn::Straight) => Facing::West,
            (Facing::West, NextTurn::Right) => Facing::North,
        };
        self.next_turn = match self.next_turn {
            NextTurn::Left => NextTurn::Straight,
            NextTurn::Straight => NextTurn::Right,
            NextTurn::Right => NextTurn::Left,
        };
    }
}

fn main() {
    let  (tracks, mut carts) =  read_data("./input.txt");
    while carts.len() > 1 {
        carts.sort_by(|a,b | a.location.cmp(&b.location));
        move_carts(&tracks, &mut carts);
    }
    let last_cart =  carts.first().unwrap();
    println!("Last remaining cart is at: ({},{})", last_cart.location.x, last_cart.location.y);
}


fn read_data(filename: &str) -> (HashMap<Coordinate, Track>, Vec<Cart>) {
    let mut tracks = HashMap::new();
    let mut carts = Vec::new();
    let data = std::fs::read_to_string(filename).unwrap();
    for (y, line) in data.split('\n').enumerate() {
        for (x, c) in line.chars().enumerate(){
            let track = match c {
                '-' => Track::EastWest,
                '|' => Track::NorthSouth,
                '\\' => Track::NorthwestSoutheast,
                '/' => Track::SouthwestNortheast,
                '+' => Track::CrossRoads,
                '>' => { 
                    carts.push(Cart { facing: Facing::East, next_turn: NextTurn::Left, location: Coordinate { x,  y } });
                    Track::EastWest
                },
                '<' => { 
                    carts.push(Cart { facing: Facing::West, next_turn: NextTurn::Left, location: Coordinate { x,  y } });
                    Track::EastWest
                },
                '^'  => { 
                    carts.push(Cart { facing: Facing::North, next_turn: NextTurn::Left, location: Coordinate { x,  y } });
                    Track::NorthSouth
                },
                'v'  => { 
                    carts.push(Cart { facing: Facing::South, next_turn: NextTurn::Left, location: Coordinate { x,  y }});
                    Track::NorthSouth
                },
                ' ' => Track::Empty,
                _ => unimplemented!()
            };
            if track != Track::Empty {
                tracks.insert(Coordinate{x,y}, track);
            }
        }
    }
    
    (tracks, carts)
}

fn move_carts(tracks: &HashMap<Coordinate,Track>, carts: &mut Vec<Cart>) {


    let mut visited = HashSet::new();
    let mut crashed = HashSet::new();

    for cart in carts.iter_mut() {
        if visited.contains(&cart.location) {
            crashed.insert(cart.location);
            continue;
        }
        match cart.facing {
            Facing::North => {cart.location.y -= 1},
            Facing::South => {cart.location.y += 1},
            Facing::East => {cart.location.x += 1},
            Facing::West => {cart.location.x -= 1},
        }
        if visited.contains(&cart.location) {
            crashed.insert(cart.location);
            continue;
        }

        match (&cart.facing, tracks.get(&cart.location).unwrap_or(&Track::Empty)) {
            (Facing::North, Track::NorthSouth) => {},
            (Facing::North, Track::NorthwestSoutheast) => {cart.facing = Facing::West},
            (Facing::North, Track::SouthwestNortheast) => {cart.facing = Facing::East},
    
            (Facing::South, Track::NorthSouth) => {},
            (Facing::South, Track::NorthwestSoutheast) => {cart.facing = Facing::East},
            (Facing::South, Track::SouthwestNortheast) => {cart.facing = Facing::West}, 
    
            (Facing::East, Track::EastWest) => {},
            (Facing::East, Track::NorthwestSoutheast) => {cart.facing = Facing::South},
            (Facing::East, Track::SouthwestNortheast) => {cart.facing = Facing::North},
    
            (Facing::West, Track::EastWest) => {},
            (Facing::West, Track::NorthwestSoutheast) => {cart.facing = Facing::North}
            (Facing::West, Track::SouthwestNortheast) => {cart.facing = Facing::South},


            (_, Track::CrossRoads) => {
                cart.turn();
            }

            (_,_) => unreachable!()
        };
        visited.insert(cart.location);
    }
    if !crashed.is_empty() {
        for location in crashed.iter() {
            println!("Crash at ({},{})", location.x, location.y);
        }
    }
    carts.retain(|c| !crashed.contains(&c.location));
}