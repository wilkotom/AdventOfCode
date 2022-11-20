use std::{collections::{HashMap, HashSet}, hash::Hash};

#[derive(Debug,Copy,Clone,Hash,Eq,PartialEq)]
enum NodeType {
    Full,
    Empty,
    Immovable,
    Goal
}

#[derive(Debug,Copy,Clone,Hash,Eq,PartialEq)]
struct Coordinate {
    x: i32,
    y: i32
}

#[derive(Debug,Copy,Clone)]
struct DiskNode {
    used: i32,
    size: i32,
    node_type: NodeType
}

fn main() {
    let data = std::fs::read_to_string("./input.txt").unwrap();
    let lines = data.split('\n').collect::<Vec<_>>();
    let mut nodes: HashMap<Coordinate, DiskNode> = HashMap::new();
    for line in &lines[2..] {
        let mut words = line.split_ascii_whitespace();
        let nodename = words.next().unwrap();
        let size_str = words.next().unwrap();
        let size = size_str[0..size_str.len()-1].parse::<i32>().unwrap();
        let used_str = words.next().unwrap();
        let used = used_str[0..used_str.len()-1].parse::<i32>().unwrap();
        let nodesplit = nodename.split('-').collect::<Vec<_>>();
        let x= nodesplit[1][1..].parse::<i32>().unwrap();
        let y= nodesplit[2][1..].parse::<i32>().unwrap();
        let node_type = if used == 0 { NodeType::Empty } else if used > 100 {NodeType::Immovable} else {NodeType::Full};
        nodes.insert(Coordinate{x,y}, DiskNode{used,size, node_type});

    }
    let mut visited: HashSet<Coordinate> = HashSet::new();
    println!("{:?}", nodes[&Coordinate{x:0, y:0}]);
    let locations = nodes.keys().cloned();
    let mut viable_pairs = 0;
    for loc in locations {
        visited.insert(loc);
        let node = nodes.get(&loc).unwrap();
        if node.used != 0 {
            for other in nodes.keys() {
                if *other != loc {
                    let other = nodes.get(&other).unwrap();
                    if other.used + node.used < other.size {
                        viable_pairs += 1
                    }
                }
            }

        }
    }
    println!("{}", viable_pairs);
    for y in 0..27 {
        for x in 0..30 {
            print!("{}", match nodes.get(&Coordinate{x,y}).unwrap().node_type {
                NodeType::Empty => '_',
                NodeType::Immovable => '#',
                NodeType::Full => '.',
                _ => unreachable!()
            });
            
        }
        println!();
    }
}
