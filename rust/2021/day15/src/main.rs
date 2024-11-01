use std::cmp::Ordering;
use std::collections::{HashMap, BinaryHeap};

#[derive(Debug,Copy,Clone,Hash,Eq,PartialEq)]
struct Coordinate {
    x: isize,
    y: isize
}

impl Coordinate {
    fn get_neighbours(self) -> Vec<Coordinate> {
        vec![Coordinate{x: self.x-1, y: self.y},Coordinate{x: self.x+1, y: self.y}, Coordinate{x: self.x, y: self.y+1}, Coordinate{x: self.x, y: self.y-1} ]
    }
}

#[derive(Debug,Copy,Clone,Hash,Eq,PartialEq)]
struct Square {
    cost: isize,
    coordinate: Coordinate
}


impl Ord for Square {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Square {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let data = std::fs::read_to_string("./input.txt").unwrap();
    let mut map: HashMap<Coordinate,isize> = HashMap::new();
    let mut max_x: isize = 0;
    let max_y = data.split('\n').count() as isize;
    for y_mul in 0..5 {
        for (y, line) in data.split('\n').enumerate() {
            max_x = line.len() as isize;
            let y_offset = y_mul * max_y;
            for x_mul in 0..5 {
                for (x, c) in line.chars().enumerate(){
                    let x_offset = x_mul * max_x;
                    let value = ((String::from(c).parse::<isize>().unwrap() + x_mul + y_mul) -1) % 9 +1;
                    map.insert(Coordinate { x: x as isize + x_offset, y: y as isize + y_offset}, value);
                }
            }
        }      
    }
    println!("Part 1: {}", shortest_route(&map, Coordinate{x: max_x-1, y:max_y-1}));
    println!("Part 2: {}", shortest_route(&map, Coordinate{x: max_x *5 -1, y:max_y*5 -1}));
}


fn shortest_route(map: &HashMap<Coordinate,isize>, goal: Coordinate) -> isize {

    let mut costs: HashMap<Coordinate, isize> = HashMap::new();
    let mut heap = BinaryHeap::new();
    
    let start = Coordinate{x:0,y:0};
    
    heap.push(Square{cost:0, coordinate: start});

    while let Some(square) = heap.pop() {
        if square.coordinate == goal{
            return square.cost;
        }
        if square.cost <= *costs.get(&square.coordinate).unwrap_or(&isize::MAX) { 
            for neighbour in square.coordinate.get_neighbours() {
                if map.contains_key(&neighbour) {
                    let next_square = Square{cost: square.cost + map.get(&neighbour).unwrap(), coordinate: neighbour};
                    if next_square.cost < *costs.get(&neighbour).unwrap_or(&isize::MAX){
                        heap.push(next_square);
                        costs.insert(neighbour, next_square.cost);
                    }
                }
            }
        }

    }
    unreachable!()
}
