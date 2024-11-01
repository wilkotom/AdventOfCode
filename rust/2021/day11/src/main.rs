use std::collections::HashMap;
use std::{thread, time};

#[derive(Debug,Copy,Clone,Hash,Eq,PartialEq)]
struct Coordinate {
    x: isize,
    y: isize
}

#[derive(Debug,Copy,Clone,Hash,Eq,PartialEq)]
struct Octopus {
        energy: i32,
        locaation: Coordinate
}
fn main() {

    let input = std::fs::read_to_string("./input.txt").unwrap();
    let mut floor_map: HashMap<Coordinate,Octopus> = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;
    let sleep_time = time::Duration::from_millis(100);

    for (y, line) in input.split('\n').enumerate() {
        max_y +=1;
        max_x = line.len() as isize;
        for (x, c) in line.chars().enumerate(){
            let loc = Coordinate { x: x as isize, y: y as isize};
            floor_map.insert(Coordinate { x: x as isize, y: y as isize}, Octopus{energy: String::from(c).parse::<i32>().unwrap(), locaation: loc});
        }
    }
    let mut steps = 0;
    let mut found = false;
    let mut flash_count = 0;
    let mut part1 = 0;
    let locations = &floor_map.keys().copied().collect::<Vec<_>>();
    print_grid(max_x, max_y, &floor_map);
    while ! found {        
        steps += 1;
        for octopus in floor_map.values_mut() {
            octopus.energy += 1;
        }
        let mut flashed = true ; 
        while flashed {
            flashed = false;
            for loc in locations {
                let mut octopus = floor_map.get_mut(loc).unwrap();
                if octopus.energy > 9 {
                    flash_count += 1;
                    octopus.energy = 0;
                    flashed = true;
                    for n in get_neighbours(loc) {
                        if floor_map.contains_key(&n) {
                            let neighbour =  floor_map.get_mut(&n).unwrap();
                            if neighbour.energy != 0 {
                                neighbour.energy +=1;
                            }
                        }
                    }    
                }
            }
        }

        thread::sleep(sleep_time);

        print!("\x1B[2J\x1B[1;1H");
        print_grid(max_x, max_y, &floor_map);
        if steps <= 100 {

            println!("Part 1: Flash count: {}", flash_count);
            part1 = flash_count;
        } else {
            println!("Part 1: Flash count: {}", part1);
        }
        println!("Part 2: Step: {}", steps);
        if floor_map.values().filter(|x| x.energy == 0).count() as isize == (max_x * max_y) {
            
            found = true;
        }
    }
}


fn get_neighbours(c: &Coordinate) -> Vec<Coordinate> {
    vec![Coordinate{x: c.x-1, y: c.y},Coordinate{x: c.x+1, y: c.y}, Coordinate{x: c.x, y: c.y+1}, Coordinate{x: c.x, y: c.y-1},
        Coordinate{x: c.x-1,y: c.y-1}, Coordinate{x: c.x+1, y: c.y+1},  Coordinate{x: c.x-1,y: c.y+1}, Coordinate{x: c.x+1, y: c.y-1}]
}


fn print_grid(max_x: isize, max_y: isize, floor_map: &HashMap<Coordinate, Octopus>) {
    let colours: HashMap<i32, &str> = HashMap::from(
        [
            (0, "\u{001b}[38;2;255;255;255m"),
            (1, "\u{001b}[38;2;25;25;25m"),
            (2, "\u{001b}[38;2;51;51;51m"),
            (3, "\u{001b}[38;2;76;76;76m"),
            (4, "\u{001b}[38;2;102;102;102m"),
            (5, "\u{001B}[38;2;128;128;128m"),
            (6, "\u{001B}[38;2;153;153;153m"),
            (7, "\u{001B}[38;2;179;179;179m"),
            (8, "\u{001B}[38;2;204;204;204m"),
            (9, "\u{001B}[38;2;230;230;230m"),
        ]
    );
    for y in 0..max_y {

        for x in 0..max_x {
            print!("{}███", colours.get(&floor_map.get(&Coordinate{x,y}).unwrap().energy).unwrap());
        } 
        println!();
        for x in 0..max_x {
            print!("{}███", colours.get(&floor_map.get(&Coordinate{x,y}).unwrap().energy).unwrap());
        } 
        
        println!("\u{001b}[38;2;255;255;255m");
    }
}