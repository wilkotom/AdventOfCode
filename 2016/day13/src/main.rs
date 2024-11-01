use std::{cmp::Ordering, collections::{BinaryHeap, HashMap, HashSet}};

const SEED: i32 = 1352;
const DEST_X: i32 = 31;
const DEST_Y: i32 =39;


#[derive(Debug,Eq,PartialEq,Hash, Copy, Clone, PartialOrd)]
struct Coordinate{
    x: i32,
    y: i32
}

#[derive(Debug, Eq,PartialEq,Hash, Copy, Clone, PartialOrd)]
struct NextNode {
    tentative_distance: i32,
    location: Coordinate
}

impl Ord for NextNode{
    fn cmp(&self, other: &Self) -> Ordering {
        self.tentative_distance.cmp(&other.tentative_distance)
    }
}

fn main() {
    println!("Steps to coordinate ({},{}): {}", DEST_X, DEST_Y, 0 - distance_to_location(Coordinate{x:DEST_X, y: DEST_Y}));
}

fn is_wall(c: Coordinate) -> bool {
    let x = c.x;
    let y = c.y;
    let mut number = x*x + 3*x + 2*x*y + y + y*y + SEED;
    let mut count = 0;
    while number >0 {
        count += if number & 1  == 1 {1} else {0};
        number >>= 1;
    }
    count %2 != 0 || x <0 || y <0
}

fn distance_to_location(dest: Coordinate) -> i32 {

    let mut tentative_distances: HashMap<Coordinate, i32> = HashMap::new();
    let mut next_nodes: BinaryHeap<NextNode> = BinaryHeap::new();
    let mut visited: HashSet<Coordinate> = HashSet::new();
    tentative_distances.insert(Coordinate{x: 1, y: 1}, 0);
    next_nodes.push(NextNode{tentative_distance: 0, location:Coordinate{x: 1, y: 1}});
    while next_nodes.len() > 0 {
        let current_node_details = next_nodes.pop().unwrap();
        if current_node_details.location == dest {
            let mut squares_within_distance = 0;
            for k in tentative_distances.keys(){
                if tentative_distances.get(k).unwrap() <= &50 {
                    squares_within_distance +=1;
                }
            }
            println!("Squares within 50 steps: {}", squares_within_distance);
            return current_node_details.tentative_distance;
        }
        let current_node = current_node_details.location;
        let current_node_distance = tentative_distances.get(&current_node).unwrap().clone();
        for node in get_neighbours(&current_node) {
            if &tentative_distances.get(&node).unwrap_or(&i32::MAX) > &&(current_node_distance + 1)  {
                tentative_distances.insert(node, current_node_distance + 1);
                next_nodes.push(NextNode{tentative_distance: 0 - (current_node_distance + 1), location: node})
            }
        }
        visited.insert(current_node);
    }
    0

}

fn get_neighbours(square: &Coordinate) -> Vec<Coordinate> {
    let x = square.x;
    let y = square.y;
    vec![Coordinate{x: x+1, y}, 
         Coordinate{x: x-1 , y},
         Coordinate{x, y:  y+ 1},
         Coordinate{x, y:  y+ -1}].iter()
      .filter(|c| !is_wall(*c.clone()))
      .map(|c| *c)
      .collect::<Vec<_>>()
}