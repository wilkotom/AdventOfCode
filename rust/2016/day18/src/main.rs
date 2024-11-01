use std::collections::HashMap;

static ROWS: usize = 400000;

#[derive(Debug,Hash,Eq,PartialEq)]
enum Tile {
    Trap,
    Safe
}
#[derive(Debug,Hash,Eq,PartialEq)]
struct Coordinate {
    x: isize,
    y: isize
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let mut floor_map: HashMap<Coordinate, Tile> = HashMap::new();
    for (x,c) in input.chars().enumerate() {
        floor_map.insert(Coordinate{x : x as isize, y:0},match c {
            '.' => Tile::Safe,
            _ => Tile::Trap
        });
    }
    for y in 1..ROWS as isize {
        for x in 0..input.len() as isize {
            let left = &floor_map.get(&Coordinate{x:x-1, y: y-1}).clone().unwrap_or(&Tile::Safe);
            let centre = &floor_map.get(&Coordinate{x, y: y-1}).clone().unwrap_or(&Tile::Safe);
            let right = &floor_map.get(&Coordinate{x: x+1, y: y-1}).clone().unwrap_or(&Tile::Safe);
            let next_square  =  if (left == centre && left != right) || (right == centre && right != left) {
                Tile::Trap
            } else {
                Tile::Safe
            };
            floor_map.insert(Coordinate{x,y}, next_square);
        }
    }
    println!("{:?}", floor_map.values().filter(|x| **x == Tile::Safe).count());
}
