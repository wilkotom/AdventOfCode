use std::collections::{HashMap, VecDeque, BinaryHeap};

#[derive(Debug,Copy,Clone,Hash,Eq,PartialEq)]
struct Coordinate {
    x: isize,
    y: isize
}
fn main() {
    let data = std::fs::read_to_string("./input.txt").unwrap();
    let (basins, map) = part1(data);
    println!("Part 2: {}", part2(basins, map));
}

fn part1(data: String) -> (Vec<Coordinate>, HashMap<Coordinate,isize>) {
    let mut map: HashMap<Coordinate,isize> = HashMap::new();
    for (y, line) in data.split("\n").enumerate() {
        for (x, c) in line.chars().enumerate(){
            map.insert(Coordinate { x: x as isize, y: y as isize}, String::from(c).parse::<isize>().unwrap());
        }
    }
    let mut basins: Vec<Coordinate> = Vec::new();
    let mut total = 0;
    for square in map.keys() {
        let mut found = true;
        let height = map.get(&square).unwrap();
        for n in get_neighbours(&square) {
            if height >= map.get(&n).unwrap_or(&isize::MAX) {
                found = false;
                break;
            }
        }
        if found {
            basins.push(*square);
            total += height +1
        }
    }
    println!("Part 1: {}", total);
    (basins, map)
}

fn part2(basins: Vec<Coordinate>, mut map: HashMap<Coordinate, isize>) -> isize{
    let mut basin_sizes: BinaryHeap<isize> = BinaryHeap::new();
    for startpoint in basins {
        let mut considered: VecDeque<Coordinate> = VecDeque::new();
        considered.push_back(startpoint);
        let mut size = 0;
        while considered.len() > 0 {
            let c = considered.pop_front().unwrap();
            if map.contains_key(&c){
                size +=1;
                
                let height = *map.get(&c).unwrap();
                map.remove(&c);
                for n in get_neighbours(&c) {
                    let nh = *map.get(&n).unwrap_or(&isize::MAX);
                    if nh > height && nh < 9{
                        considered.push_back(n);
                    }
                }
            }
        }
        basin_sizes.push(size);
    }
    basin_sizes.pop().unwrap() * basin_sizes.pop().unwrap() * basin_sizes.pop().unwrap()
}

fn get_neighbours(c: &Coordinate) -> Vec<Coordinate> {
    vec![Coordinate{x: c.x-1, y: c.y},Coordinate{x: c.x+1, y: c.y}, Coordinate{x: c.x, y: c.y+1}, Coordinate{x: c.x, y: c.y-1} ]
}