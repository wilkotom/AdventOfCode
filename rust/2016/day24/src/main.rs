use std::collections::{HashMap, VecDeque, HashSet};
use itertools::Itertools;


#[derive(Copy,Clone,Debug, PartialEq, Eq, Hash)]
struct Coordinate {
    x: i32,
    y: i32
}

impl Coordinate {
    fn get_neighbours(self) -> Vec<Coordinate> {
        let x = self.x;
        let y = self.y;
        vec![Coordinate{x: x+1, y}, 
             Coordinate{x: x-1 , y},
             Coordinate{x, y:  y+ 1},
             Coordinate{x, y:  y+ -1}]
    }
}

#[derive(Copy,Clone,Debug, PartialEq, Eq, Hash)]
enum Destination {
    Empty,
    Location(char)
}

fn main() {
    let data = std::fs::read_to_string("./input.txt").unwrap();
    let mut maze_map: HashMap<Coordinate, Destination> = HashMap::new();
    let mut interesting_locations: HashMap<char, Coordinate> = HashMap::new();
    for (y, line) in data.split('\n').enumerate() {
        for (x, c) in line.chars().enumerate(){
            match c {
                '.' => {maze_map.insert(Coordinate{x:x as i32,y:y as i32}, Destination::Empty);}
                '0'..='9' => {
                    maze_map.insert(Coordinate{x:x as i32,y:y as i32}, Destination::Location(c));
                    interesting_locations.insert(c, Coordinate{x:x as i32,y:y as i32});
                }
                _ => {}
            }
        }
    }
    let mut distances_from: HashMap<char, HashMap<Coordinate, usize>> = HashMap::new();
    for key in interesting_locations.keys().collect::<Vec<_>>() {
        let distances = gen_distance_map(&maze_map, interesting_locations[key]);
        distances_from.insert(*key, distances);
    }
    let perms = interesting_locations.keys().filter(|x| **x != '0').copied().collect::<Vec<_>>();
    let len = perms.len();
    let mut min_distance = usize::MAX;
    for mut perm in perms.into_iter().permutations(len) {
        perm.insert(0, '0');
        let mut total = 0;
        for i in 0..perm.len()-1 {
            let start = perm[i];
            let end = perm[i+1];
            let distance = distances_from[&start].get(&interesting_locations[&end]).unwrap();
            total += distance;
        }
        // println!("{:?} {}", perm, total);
        min_distance = min_distance.min(total);
    }
    println!("{}", min_distance);

    let perms = interesting_locations.keys().filter(|x| **x != '0').copied().collect::<Vec<_>>();
    let len = perms.len();
    let mut min_distance = usize::MAX;
    for mut perm in perms.into_iter().permutations(len) {
        perm.insert(0, '0');
        perm.push('0');
        let mut total = 0;
        for i in 0..perm.len()-1 {
            let start = perm[i];
            let end = perm[i+1];
            let distance = distances_from[&start].get(&interesting_locations[&end]).unwrap();
            total += distance;
        }
        // println!("{:?} {}", perm, total);
        min_distance = min_distance.min(total);
    }
    println!("{}", min_distance);

}

fn gen_distance_map(maze_map: &HashMap<Coordinate, Destination>, start_point: Coordinate) -> HashMap<Coordinate, usize> {
    let mut distance_map: HashMap<Coordinate, usize> = HashMap::new();
    let mut unconsidered: VecDeque<(Coordinate, usize)> = VecDeque::new();
    let mut queued: HashSet<Coordinate> = HashSet::new();
    unconsidered.push_back((start_point, 0));
    queued.insert(start_point);
    while !unconsidered.is_empty() {
        let (point, distance) = unconsidered.pop_front().unwrap();
        // println!("{:?} {} {} {}", point, distance, distance_map.len(), maze_map.len());
        distance_map.insert(point, distance);
        for neighbour in point.get_neighbours() {
            if maze_map.contains_key(&neighbour) && !distance_map.contains_key(&neighbour) && !queued.contains(&neighbour){
                    unconsidered.push_back((neighbour, distance+1));
                    queued.insert(neighbour);
                
            }
        }

    }
    distance_map

}


