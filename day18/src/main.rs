use std::{fs::read_to_string, collections::HashMap};

#[derive(Debug,Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
    x: i32,
    y: i32
}

impl Coordinate {
    fn get_neighbours(&self) -> Vec<Coordinate> {
        vec![
            Coordinate{x: self.x-1, y: self.y-1},
            Coordinate{x: self.x,   y: self.y-1 },
            Coordinate{x: self.x+1, y: self.y-1},

            Coordinate{x: self.x-1, y: self.y},
            Coordinate{x: self.x+1, y: self.y},
            
            Coordinate{x: self.x-1, y: self.y+1},
            Coordinate{x: self.x,   y: self.y+1},
            Coordinate{x: self.x+1, y: self.y+1},
        ]
    }
}

fn main() {
    let data = read_to_string("./input.txt").unwrap();
    let mut grid: HashMap<Coordinate, bool> = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;
    let part2 = true;
    for (y, line) in data.split('\n').enumerate() {
        max_y = max_y.max(y as i32);
        for (x,c) in line.chars().enumerate() {
            max_x = max_x.max(x as i32);
            grid.insert(Coordinate{x: x as i32 ,y: y as i32}, c == '#');
        }
    }
    grid.insert(Coordinate{x:0, y:0}, true);
    grid.insert(Coordinate{x:max_x, y:0}, true);
    grid.insert(Coordinate{x:0, y:max_y}, true);
    grid.insert(Coordinate{x:max_y, y:max_y}, true);

    for _ in 0..100 {
        grid = generation(grid);
        if part2 {
            grid.insert(Coordinate{x:0, y:0}, true);
            grid.insert(Coordinate{x:max_x, y:0}, true);
            grid.insert(Coordinate{x:0, y:max_y}, true);
            grid.insert(Coordinate{x:max_y, y:max_y}, true);
        }
    }
    println!("{}", grid.values().filter(|v| **v).count());
}

fn generation(grid: HashMap<Coordinate, bool>) -> HashMap<Coordinate, bool> {
    let mut next_grid = HashMap::new();
    for bulb in grid.keys() {
        let counter = bulb.get_neighbours().iter().filter(|b| *grid.get(b).unwrap_or(&false)).count();
        next_grid.insert(*bulb, counter == 3 || counter == 2 && grid[bulb]);
    }

    next_grid
}