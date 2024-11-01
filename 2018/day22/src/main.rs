use std::{collections::{HashMap, BinaryHeap}, cmp::Ordering};

#[derive(Debug,Clone,Copy, Hash, PartialEq, Eq)]
struct Location {
    x: i32,
    y: i32,
}

impl Location {
    fn get_neighbours(&self) -> Vec<Self> {
        vec![Location{x: self.x-1, y: self.y},
             Location{x: self.x+1, y: self.y}, 
             Location{x: self.x, y: self.y+1}, 
             Location{x: self.x, y: self.y-1} ]
    }
}

#[derive(Debug,Copy,Clone,Hash,Eq,PartialEq)]
enum Equipment {
    ClimbingGear,
    Torch,
    Nothing
}

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
enum Region{
    Rocky,
    Wet,
    Narrow,
    Solid
}

#[derive(Debug,Copy,Clone,Hash,Eq,PartialEq)]
struct Position {
    elapsed: i32,
    place: Location,
    holding: Equipment
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        other.elapsed.cmp(&self.elapsed)
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
fn main() {
    let depth = 11991;
    let target_x = 6;
    let target_y = 797;
    let target = Location{x:target_x, y: target_y};

    let mut cave_map = HashMap::new();
 
    for y in 0.. (target.y) +71 {
        for x in 0..(10*target.x) +71 {
            cave_map.insert(Location{x,y}, match (x,y) {
                (0,_) => { y * 48271},
                (_,0) => { x * 16807},
                _ => {
                    ((cave_map.get(&Location{x:x-1, y}).unwrap() + depth) % 20183) *
                    ((cave_map.get(&Location{x, y:y-1}).unwrap() + depth) % 20183) 
                }
            });
        }
    }

    println!("Part 1: {}", part1(&cave_map, target.x, target.y, depth));
    let cave_map = build_map(&cave_map, target.x +70, target.y+70, depth);
    let starting_position = Position{elapsed: 0, place: Location{x:0, y:0}, holding: Equipment::Torch};

    let mut shortest = HashMap::new();
    let mut next_moves = BinaryHeap::new();
    next_moves.push(starting_position);

    while !next_moves.is_empty() {
        let current_move = next_moves.pop().unwrap();
        if shortest.contains_key(&(current_move.place,current_move.holding)) {
            continue;
        }

        if current_move.place.x == target_x && current_move.place.y == target_y && current_move.holding == Equipment::Torch {
            println!("Part 2: {}", current_move.elapsed);
            break;
        }
        shortest.insert((current_move.place,current_move.holding), current_move.elapsed);
        let neighbours = current_move.place.get_neighbours();
        for neighbour in neighbours {
            let region = *cave_map.get(&neighbour).unwrap_or(&Region::Solid);
            match current_move.holding {
                Equipment::ClimbingGear => {
                    if region == Region::Rocky || region == Region::Wet {
                        next_moves.push( Position{elapsed: current_move.elapsed+1, place: neighbour, holding: current_move.holding})
                    }
                },
                Equipment::Torch => {
                    if region == Region::Narrow || region == Region::Rocky {
                        next_moves.push( Position{elapsed: current_move.elapsed+1, place: neighbour, holding: current_move.holding})
                    }
                },
                Equipment::Nothing => {
                    if region == Region::Narrow || region == Region::Wet {
                        next_moves.push( Position{elapsed: current_move.elapsed+1, place: neighbour, holding: current_move.holding})
                    }
                },
            };
        }
        match cave_map.get(&current_move.place).unwrap() {
            Region::Rocky => {
                if current_move.holding == Equipment::ClimbingGear{
                    next_moves.push(Position{elapsed: current_move.elapsed+7, place: current_move.place, holding: Equipment::Torch})
                } else {
                    next_moves.push(Position{elapsed: current_move.elapsed+7, place: current_move.place, holding: Equipment::ClimbingGear})
                }
            }

            Region::Wet => {
                if current_move.holding == Equipment::ClimbingGear{
                    next_moves.push(Position{elapsed: current_move.elapsed+7, place: current_move.place, holding: Equipment::Nothing})
                } else {
                    next_moves.push(Position{elapsed: current_move.elapsed+7, place: current_move.place, holding: Equipment::ClimbingGear})
                }
            },
            Region::Narrow => {
                if current_move.holding == Equipment::Torch{
                    next_moves.push(Position{elapsed: current_move.elapsed+7, place: current_move.place, holding: Equipment::Nothing})
                } else {
                    next_moves.push(Position{elapsed: current_move.elapsed+7, place: current_move.place, holding: Equipment::Torch})
                }
            }
            Region::Solid => unreachable!()
        }
    }

}


fn build_map(map: &HashMap<Location,i32>, max_x: i32, max_y: i32, depth: i32) -> HashMap<Location, Region>{
    let mut final_map = HashMap::new();
    for y in 0..max_y +1{
        for x in 0..max_x +1 {
            final_map.insert(Location{x,y}, match ((map.get(&Location{x,y}).unwrap() + depth) % 20183) % 3 {
                0 => Region::Rocky,
                1 => Region::Wet,
                2 => Region::Narrow,
                _ => unreachable!()
            });
        }
    }
    final_map.insert(Location{x: max_x,y: max_y}, Region::Rocky);
    final_map
}


fn part1(map: &HashMap<Location,i32>, max_x: i32, max_y: i32, depth: i32) -> i32 {
    let mut total = 0;
    for y in 0..max_y +1{
        for x in 0..max_x +1 {
           total +=  ((map.get(&Location{x,y}).unwrap() + depth) % 20183) % 3;
        }
    }
    total
}
