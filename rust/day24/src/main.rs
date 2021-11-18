use std::fs;
use std::collections::HashMap;

#[derive(Copy,Clone,PartialEq,Eq,Hash,Debug)]
struct Coordinate {
    x: i64,
    y: i64
}


fn main() {
    let directions = fs::read_to_string("./input.txt").unwrap_or("".to_string()).split("\n").map(|x| x.to_string()).collect::<Vec<String>>();
    let generations = 100;
    let mut floor: HashMap<Coordinate,bool> = HashMap::new();

    for direction in directions{
        let mut steps = direction.chars().rev().collect::<Vec<char>>();
        let mut x: i64 = 0;
        let mut y: i64 = 0;
        while steps.len() > 0 {
            let next = steps.pop().unwrap();
            if next == 'e' {
                x += 2
            } else if next == 'w' {
                x -= 2;
            } else {
                if next == 'n' {
                    y +=1;
                } else {
                    y -= 1;
                }
                let next = steps.pop().unwrap();
                if next == 'e' {
                    x+=1;
                } else {
                    x -=1;
                }
            }
        }
        if !floor.contains_key(&Coordinate{x,y}) {
            floor.insert(Coordinate{x,y}, true);
        } else {
            floor.remove(&Coordinate{x,y});
        }
    }

    println!("Part 1: {}", floor.len());
    for _ in 0..generations {
        floor = get_next_generation(floor);
    }
    println!("Part 2: {}", floor.len());

}

fn get_next_generation(mut floor: HashMap<Coordinate,bool>) -> HashMap<Coordinate,bool> {
    let mut next_floor: HashMap<Coordinate,bool> = HashMap::new();
    let known_tiles = &floor.keys().cloned().collect::<Vec<_>>();
    for black_tile in known_tiles.iter() {
        for tile in get_neighbours(&black_tile) {
            if ! floor.contains_key(&tile) {
                floor.insert(tile, false);
            }
        }
    }

    for tile in floor.keys() {
        let neighbour_count = get_neighbours(&tile).iter().filter(|x| *floor.get(&x).unwrap_or(&false)).count();
        if neighbour_count == 2 || neighbour_count == 1 && *floor.get(&tile).unwrap_or(&false) {
            next_floor.insert(*tile, true);
        }
    }

    next_floor
}

fn get_neighbours(c: &Coordinate) -> Vec<Coordinate> {
    vec![Coordinate{x: c.x+2, y: c.y}, 
         Coordinate{x: c.x-2, y: c.y}, 
         Coordinate{x: c.x+1, y: c.y+1}, 
         Coordinate{x: c.x+1, y: c.y-1},
         Coordinate{x: c.x-1, y: c.y+1},
         Coordinate{x: c.x-1, y: c.y-1}]

}