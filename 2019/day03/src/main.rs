use std::{fs::read_to_string, collections::HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coordinate {
    x: i32,
    y: i32
}

fn main() {
    let data = read_to_string("./day03/input.txt").unwrap();
    let mut lines = data.split('\n');
    let red = board_map(lines.next().unwrap());
    let black = board_map(lines.next().unwrap());
    println!("Part 1: {}", part1(&red, &black));
    println!("Part 2: {}", part2(&red, &black));
}

fn part1(red: &HashMap<Coordinate, i32>, black: &HashMap<Coordinate,i32>) -> i32 {

    let mut max_dist = i32::MAX;
    for red_coord in red.keys() {
       if black.contains_key(red_coord){
            let distance = red_coord.x.abs() + red_coord.y.abs();
            if distance > 0 && distance < max_dist {
                max_dist = distance;
            }
        
        }
    }
    max_dist
}

fn part2(red: &HashMap<Coordinate, i32>, black: &HashMap<Coordinate,i32>) -> i32 {
    let mut max_dist = i32::MAX;
    for red_coord in red.keys() {
       if black.contains_key(red_coord){
            let distance = red[red_coord] + black[red_coord];
            if distance > 0 && distance < max_dist {
                max_dist = distance;
            }
        
        }
    }
    max_dist
}

fn board_map( line: &str) -> HashMap<Coordinate, i32> {
    let mut grid = HashMap::new();
    let mut current_point = Coordinate{x:0, y:0};
    let mut current_distance = 0;
    for point in line.split(',') {
        let mut length = point[1..].parse::<i32>().unwrap();
        while length >0 {
            grid.entry(current_point).or_insert(current_distance);    
            length -=1;
            current_distance +=1;
            current_point = match point.chars().next() {
                Some('L') => {
                    Coordinate{x: current_point.x -1 , y: current_point.y}
                }
                Some('R') => {
                    Coordinate{x: current_point.x + 1, y: current_point.y}
                }
                Some('D') => {
                    Coordinate{x: current_point.x, y: current_point.y - 1}
                }
                Some('U') => {
                    Coordinate{x: current_point.x, y: current_point.y + 1}
                }
                _ => unimplemented!()
            };
        }
    } 
    grid
}