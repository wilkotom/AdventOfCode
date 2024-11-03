use std::{fs::read_to_string, collections::HashMap};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Coordinate {
    x: i32,
    y: i32
}

#[derive (Debug, Copy, Clone, Eq, PartialEq)]
enum Object {
    Clay,
    MovingWater,
    StillWater,
    Empty
}

struct Grid {
    grid: HashMap<Coordinate, Object>
}

impl Grid {
    fn _display(&self) {
        let min_x = self.grid.keys().map(|n| n.x).min().unwrap();
        let max_x = self.grid.keys().map(|n| n.x).max().unwrap();
        let max_y = self.grid.keys().map(|n| n.y).max().unwrap();

        for y in 0..max_y +1 {
            for x in min_x-1 .. max_x+2 {
                match self.grid.get(&Coordinate{x,y}).unwrap_or(&Object::Empty) {
                    Object::Clay => print!("#"),
                    Object::MovingWater => print!("|"),
                    Object::StillWater => print!("~"),
                    Object::Empty => print!("."),
                }
            }
            println!();
        }
    }

    fn max_y(&self) -> i32 {
        self.grid.keys().map(|n| n.y).max().unwrap()
    }
    fn min_y(&self) -> i32 {
        self.grid.keys().map(|n| n.y).min().unwrap()
    }
}


fn main() {
    let mut grid = read_grid("./input.txt");
    let mut next = vec![Coordinate{x:500, y:grid.min_y()}];
    
    while let Some(location) = next.pop() {
        
        if location.y > grid.max_y() {
            continue;
        }
        let below = Coordinate{x: location.x,  y: location.y+1};
        match grid.grid.get(&below).unwrap_or(&Object::Empty) {
            Object::Clay | Object::StillWater => {
                let mut left_edge = location.x;
                let mut right_edge = location.x;
                while grid.grid.get(&Coordinate{x: left_edge -1, y: location.y}) != Some(&Object::Clay) {
                    left_edge -= 1;
                    if !grid.grid.contains_key(&Coordinate{x: left_edge, y: location.y +1}) ||  grid.grid.get(&Coordinate{x: left_edge, y: location.y +1}) == Some(&Object::MovingWater) {
                        break;
                    }
                }

                while grid.grid.get(&Coordinate{x: right_edge +1, y: location.y}) != Some(&Object::Clay) {
                    right_edge += 1;
                    if !grid.grid.contains_key(&Coordinate{x: right_edge, y: location.y +1}) ||  grid.grid.get(&Coordinate{x: right_edge, y: location.y +1}) == Some(&Object::MovingWater) {
                        break;
                    }
                }
                let fill = if grid.grid.get(&Coordinate{x: left_edge -1, y: location.y}) == Some(&Object::Clay) && grid.grid.get(&Coordinate{x: right_edge +1, y: location.y}) == Some(&Object::Clay) {
                    Object::StillWater } else { Object::MovingWater };
                for x in left_edge ..= right_edge {
                    grid.grid.insert(Coordinate{x, y: location.y}, fill);
                }
                if fill == Object::StillWater {
                    next.push(Coordinate{x: location.x,  y: location.y-1});
                } else {
                    if !grid.grid.contains_key(&Coordinate{x: right_edge, y: location.y+1}) {
                        next.push(Coordinate{x: right_edge, y: location.y+1});
                    }
                    if !grid.grid.contains_key(&Coordinate{x: left_edge, y: location.y+1}) {
                        next.push(Coordinate{x: left_edge, y: location.y+1});
                    }                
                }

            },
            Object::MovingWater => {grid.grid.insert(location, Object::MovingWater);},
            Object::Empty => {
                next.push(below);
                grid.grid.insert(location, Object::MovingWater);
            },
        }
    }
    // grid.display();
    println!("Part 1: {}", grid.grid.values().filter(|x| x != &&Object::Clay).count());
    println!("Part 2: {}", grid.grid.values().filter(|x| x == &&Object::StillWater).count());
}


fn read_grid(filename: &str) -> Grid {
    let data = read_to_string(filename).unwrap();
    let mut grid: HashMap<Coordinate, Object> = HashMap::new();
    for line in data.split('\n') {
        let mut words= line.split(", ");

        let first = &words.next().unwrap()[2..].parse::<i32>().unwrap();
        let second = &words.next().unwrap();
        let mut bounds = second[2..].split("..").map(|n| n.parse::<i32>().unwrap());
        if second.starts_with('y') {
            for y in bounds.next().unwrap()..bounds.next().unwrap()+1 {
                grid.insert(Coordinate{x: *first, y}, Object::Clay);
            }
        } else {
            for x in bounds.next().unwrap()..bounds.next().unwrap()+1 {
                grid.insert(Coordinate{x, y : *first}, Object::Clay);
            }

        }
    }

    Grid{grid}
}