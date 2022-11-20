use std::collections::VecDeque;

use hashbrown::{HashMap, HashSet};


#[derive(Debug,Copy,Clone,Eq,PartialEq,Hash)]
struct Coordinate {
    x: i32,
    y: i32
}

impl Coordinate {
    fn get_neighbours(&self) -> Vec<Coordinate> {
        vec![   Coordinate{x: self.x-1, y:self.y}, 
                Coordinate {x: self.x+1, y: self.y}, 
                Coordinate{x: self.x, y:self.y-1}, 
                Coordinate{x: self.x, y: self.y+1}
        ]
    }
}

fn main() {
    let mut counter = 0;
    let mut grid: HashMap<Coordinate,bool> = HashMap::new();
    for y in 0..128 {
        let result = knot_hash(&format!("vbqugkhl-{}",y));
        counter += result.chars().filter(|c| *c == '1').count();
        for (x, c) in result.chars().enumerate() {
            grid.insert(Coordinate{x: x as i32, y: y as i32}, c == '1');
        }
    }
    println!("Part 1: {}", counter);

    let mut visited: HashSet<Coordinate> = HashSet::new();
    let mut part2_count = 0;
    for y in 0..128 {
        for x in 0..128 {
            let starting_point = Coordinate{x,y};
            let mut points: VecDeque<Coordinate> = VecDeque::new();        
            if grid[&starting_point] && ! visited.contains(&starting_point) {
                points.push_back(starting_point);
                while ! points.is_empty() {
                    let next = points.pop_front().unwrap();
                    if visited.contains(&next) {
                        continue;
                    }
                    visited.insert(next);
                    if *grid.get(&next).unwrap_or(&false) {
                        for neighbour in next.get_neighbours() {
                            points.push_back(neighbour);
                        }
                
                    }
                }
                part2_count +=1
            }

        }
    }
    println!("{}", part2_count);
}


fn knot_hash(length_sequence: &str) -> String{
    let mut list = (0..256).collect::<Vec<_>>();
    let list_len = list.len();
    let mut position = 0;
    let mut skip_size = 0;

    let mut inputs = length_sequence.chars().map(|c| c as usize).collect::<Vec<_>>();
    inputs.append(&mut vec![17,31,73,47,23]);

    for _ in 0..64 {
        for input in &inputs {

            let start = position % list.len();
            let mut new_list: Vec<i32> = Vec::new();
            for i in start .. (start + input) {
                new_list.push(list[i % list.len()]);
            }
            for i in start .. (start + input) {
                list[i % list_len] = new_list.pop().unwrap();
            }
            position = position + skip_size + input;
            skip_size +=1;
        }
    }
    let mut output = "".to_owned();
    for i in 0..16 {
        
        let mut running = 0;
        for j in 0..16 {
            running ^= list[i*16+j];
        }
        // println!("{:08b}", running);
        output.push_str(&format!("{:08b}", running));
    }
    // println!("{}", output);
    output

}