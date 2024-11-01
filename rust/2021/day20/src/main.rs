use std::fs::read_to_string;
use hashbrown::HashSet;

#[derive(Debug,Copy,Clone,Hash,Eq,PartialEq)]
struct Coordinate {
    x: isize,
    y: isize
}

impl Coordinate {
    fn get_window(self) -> Vec<Coordinate> {
        vec![ 
            Coordinate{x: self.x -1, y: self.y-1},
            Coordinate{x: self.x,    y: self.y-1},
            Coordinate{x: self.x +1, y: self.y-1},
            Coordinate{x: self.x-1,  y: self.y},
            Coordinate{x: self.x,    y: self.y},
            Coordinate{x: self.x+1,  y: self.y},
            Coordinate{x: self.x -1, y: self.y+1},
            Coordinate{x: self.x,    y: self.y+1},
            Coordinate{x: self.x +1, y: self.y+1},
            ]
    }
}

enum GridType {
    LitPixels,
    UnlitPixels
}

const GENERATIONS: i32 = 50;


fn main() {
    let input = read_to_string("./input.txt").unwrap();
    let mut sections = input.split("\n\n");
    let mut lookup: Vec<bool> = Vec::new();
    for c in sections.next().unwrap().chars() {
        lookup.push(matches!(c, '#'));
    }
    let mut grid: HashSet<Coordinate> = HashSet::new();
    let grid_text = sections.next().unwrap();
    for (y, line) in grid_text.split('\n').enumerate(){
        for (x, c) in line.chars().enumerate(){
            if c == '#' {
                grid.insert(Coordinate{x: x as isize,y: y as isize});
            }
        }
    }
    let mut grid_type = GridType::LitPixels;
    for _ in 0..GENERATIONS {
        let res = generation(grid, &lookup, grid_type);
        grid = res.0;
        grid_type = res.1;
    }
    println!("{}", grid.len());
}


fn generation(grid: HashSet<Coordinate>, lookup: &[bool], grid_type: GridType) -> (HashSet<Coordinate>, GridType) {
    
    let mut min_y = isize::MAX;
    let mut min_x = isize::MAX;
    let mut max_y = isize::MIN;
    let mut max_x = isize::MIN;

    let mut result: HashSet<Coordinate> = HashSet::new();

    for key in &grid {
        let Coordinate{x,y} = key;
        min_y = min_y.min(*y);
        min_x = min_x.min(*x);
        max_y = max_y.max(*y);
        max_x = max_x.max(*x);
        
    }

    for y in min_y-1..max_y+2 {
        for x in min_x-1..max_x+2 {
            let centre = Coordinate{x,y};
            let window = centre.get_window();
            let mut result_num = 0;
            for coord in window {
                result_num <<= 1;
                match grid_type {
                    GridType::LitPixels => {
                        if grid.contains(&coord) {
                            result_num+=1;
                        }
                    }
                    GridType::UnlitPixels => {
                        if !grid.contains(&coord) {
                            result_num+=1;
                        }

                    }
                }
            }
            match grid_type {
                GridType::LitPixels => {
                    if ! lookup[0] {
                        if lookup[result_num] {
                            result.insert(centre);
                        }
                    } else if !lookup[result_num] {
                        result.insert(centre);
                    } 
                }
                GridType::UnlitPixels => {
                    if lookup[result_num] {
                        result.insert(centre);
                    }
                }
            }
        }
    }
    let result_type = if !lookup[0] { GridType::LitPixels } else {
        match grid_type {
            GridType::LitPixels => GridType::UnlitPixels,
            GridType::UnlitPixels => GridType::LitPixels
        }
    };
    (result, result_type)
}

fn print_grid(grid: &HashSet<Coordinate>) {
    let mut min_y = isize::MAX;
    let mut min_x = isize::MAX;
    let mut max_y = isize::MIN;
    let mut max_x = isize::MIN;
    for key in grid {
        let Coordinate{x,y} = key;
        min_y = min_y.min(*y);
        min_x = min_x.min(*x);
        max_y = max_y.max(*y);
        max_x = max_x.max(*x);
        
    }
    for y in min_y..max_y+1 {
        for x in min_x..max_x+1 {
            if grid.contains(&Coordinate{x,y}) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}