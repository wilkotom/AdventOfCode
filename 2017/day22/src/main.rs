use std::collections::{HashSet, HashMap};

#[derive(Hash,PartialEq, Eq,Debug,Clone, Copy)]
struct Coordinate {
    x: i32,
    y: i32
}

enum NodeState {
    Clean,
    Weakened,
    Infected,
    Flagged
}

#[derive(Debug)]
struct Carrier{
    location: Coordinate,
    facing: Coordinate
}

impl Carrier{
    fn turn_right(&mut self) {
        self.facing = Coordinate{x: -self.facing.y, y: self.facing.x};
    }

    fn turn_left(&mut self) {
        self.facing = Coordinate{x: self.facing.y, y: -self.facing.x};
    }

    fn step(&mut self) {
        self.location.x += self.facing.x;
        self.location.y += self.facing.y;
    }
}

fn main() {
    let data = std::fs::read_to_string("./input.txt").unwrap();
    part1(&data);
    part2(&data);

}

fn part1(data: &str) {
    let mut map: HashSet<Coordinate> = HashSet::new();
    for (y, line) in data.split('\n').enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                map.insert(Coordinate{x:x as i32,y:y as i32});
            }
        }
    }

    let mut carrier = Carrier{location: Coordinate{x: (data.split('\n').next().unwrap().chars().count() /2) as i32, y:  (data.split('\n').count() /2 )as i32}, facing: Coordinate{x:0, y: -1} };
    let mut infected_count = 0;

    for _ in 0..10000 {
        if map.contains(&carrier.location) {
            map.remove(&carrier.location);
            carrier.turn_right();
        } else {
            map.insert(carrier.location);
            carrier.turn_left();
            infected_count +=1
        }
        carrier.step()
    }
    println!("Part 1: {}", infected_count);

}


fn part2(data: &str) {
    let mut map: HashMap<Coordinate, NodeState> = HashMap::new();
    for (y, line) in data.split('\n').enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                map.insert(Coordinate{x:x as i32,y:y as i32}, NodeState::Infected);
            }
        }
    }

    let mut carrier = Carrier{location: Coordinate{x: (data.split('\n').next().unwrap().chars().count() /2) as i32, y:  (data.split('\n').count() /2 )as i32}, facing: Coordinate{x:0, y: -1} };
    let mut infected_count = 0;

    for _ in 0..10000000 {
        match map.get(&carrier.location) {
            Some(&NodeState::Weakened) => {
                map.insert(carrier.location, NodeState::Infected);
                infected_count +=1;
            },
            Some(&NodeState::Infected) => {
                map.insert(carrier.location, NodeState::Flagged);
                carrier.turn_right();
            },
            Some(&NodeState::Flagged) => {
                map.remove(&carrier.location);
                carrier.turn_right();
                carrier.turn_right();
            }
            None => {
                map.insert(carrier.location, NodeState::Weakened);
                carrier.turn_left();
            },
            _ => {}
        }

        carrier.step()
    }
    println!("Part 2: {}", infected_count);

}
