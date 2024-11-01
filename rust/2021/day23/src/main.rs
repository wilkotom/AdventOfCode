use std::{collections::BinaryHeap, cmp::Ordering};
use hashbrown::HashSet;

#[derive(Copy,Clone,Debug, PartialEq, Eq, Hash)]
enum Amphipod {
    Amber,
    Bronze, 
    Copper,
    Desert
}

#[derive(Copy,Clone,Debug, PartialEq, Eq, Hash)]
enum Square {
    Empty,
    Occupied(Amphipod),
    Forbidden
}

#[derive(Clone,Debug, PartialEq, Eq, Hash)]
struct GameState {
    cost: isize,
    corridor: Vec<Square>,
    rooms: Vec<Vec<Amphipod>>
}

impl Ord for GameState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for GameState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


fn main() {
    let data = std::fs::read_to_string("./test.txt").unwrap();
    let mut lines = data.split('\n');
    let _ = lines.next();
    let corridor_str = lines.next().unwrap();
    let mut corridor = Vec::new();
    for c in corridor_str.chars() {
        corridor.push(
            match c {
                '#' => Square::Forbidden,
                '.' => Square::Empty,
                'A' => Square::Occupied(Amphipod::Amber),
                'B' => Square::Occupied(Amphipod::Bronze),
                'C' => Square::Occupied(Amphipod::Copper),
                'D' => Square::Occupied(Amphipod::Desert),
                _ => unreachable!()
            }
        )
    }
    for i in [3,5,7,9] {
        corridor[i] = Square::Forbidden;
    }
    let mut rooms: Vec<Vec<Amphipod>> = vec![vec![], vec![], vec![], vec![]];
    let mut room_capacity = 0;
    'outer: for line in lines {
        for pos in [3,5,7,9] {

            match line.chars().nth(pos) {
                Some('#') => break 'outer,
                Some('.') => {}
                Some('A') => {rooms[(pos-1) / 2 -1].insert(0,Amphipod::Amber)},
                Some('B') => {rooms[(pos-1) / 2 -1].insert(0,Amphipod::Bronze)},
                Some('C') => {rooms[(pos-1) / 2 -1].insert(0,Amphipod::Copper)},
                Some('D') => {rooms[(pos-1) / 2 -1].insert(0,Amphipod::Desert)},
                _ => unreachable!(),
            };
        }
        room_capacity +=1;
    }

    let starting_state = GameState{ cost: 0, corridor, rooms};


    let mut heap = BinaryHeap::new();
    heap.push(starting_state);

    let bin_requirements = vec![Amphipod::Amber, Amphipod::Bronze, Amphipod::Copper, Amphipod::Desert];

    let mut seen: HashSet<(Vec<Square>, Vec<Vec<Amphipod>>)> = HashSet::new();
    while let Some(state) = heap.pop() {

        if seen.contains(&(state.corridor.clone(), state.rooms.clone())) {
            continue;
        } else if is_winning_position(&state.rooms, room_capacity as usize) {
            println!("Winner! {}", state.cost);
            return;
        } 
        seen.insert((state.corridor.clone(), state.rooms.clone()));
        
        for (i, n) in state.corridor.iter().enumerate() {
            
            if let Square::Occupied(amphipod) = n {
                let target_room_number = match &amphipod {
                    Amphipod::Amber => 0,
                    Amphipod::Bronze => 1,
                    Amphipod::Copper => 2,
                    Amphipod::Desert => 3,
                };
                let target_room = state.rooms.get(target_room_number).unwrap();

                if target_room.is_empty() || target_room.iter().filter(|a| a != &amphipod).count() == 0 {

                    let mut reachable = true;
                    let target_index = (target_room_number+1) *2 +1;
                    if i < target_index {
                        for n in i+1 .. target_index + 1 {
                            if state.corridor[n] != Square::Forbidden && state.corridor[n] != Square::Empty {
                                reachable = false;
                            }
                        }
                    } else {
                        for n in (target_index ..i ).rev() {
                            if state.corridor[n] != Square::Forbidden && state.corridor[n] != Square::Empty {
                                reachable = false;
                            }
                        }
                    }
                    
                    if reachable {
                        let steps =  ((target_room_number as isize +1) * 2 + 1 - i as isize).abs() + (room_capacity - target_room.len() as isize);
                        let score = steps * match &amphipod {
                            Amphipod::Amber => 1,
                            Amphipod::Bronze => 10,
                            Amphipod::Copper => 100,
                            Amphipod::Desert => 1000,
                        };
                        let mut new_state = state.clone();
                        new_state.corridor[i] = Square::Empty;
                        new_state.rooms[target_room_number].push(*amphipod);
                        new_state.cost += score;
                        heap.push(new_state);
                    }
                }
            }
        }

        for (i, bin) in state.rooms.iter().enumerate() {
            if let Some(top_item) = bin.last() {
                if top_item != &bin_requirements[i] || bin.iter().filter(|x| *x != &bin_requirements[i]).count()  > 0 {
                    let corridor_point = (i+1) * 2 +1;
                    let mut potential_destinations: Vec<usize> = Vec::new();
                    for n in (1..corridor_point).rev() {
                        if state.corridor[n] == Square::Empty {
                            potential_destinations.push(n);
                        } else if state.corridor[n] != Square::Forbidden {
                            break;
                        }
                    }

                    for n in corridor_point..12 {
                        if state.corridor[n] == Square::Empty {
                            potential_destinations.push(n);
                        } else if state.corridor[n] != Square::Forbidden {
                            break;
                        }
                    }
                    for d in potential_destinations {
                        let mut new_state = state.clone();
                        let mut move_cost = room_capacity - bin.len() as isize +1;
                        let moved = new_state.rooms[i].pop().unwrap();
                        let distance = (corridor_point as isize - d as isize).abs();
                        move_cost += distance;
                        move_cost *=  match &moved {
                            Amphipod::Amber => 1,
                            Amphipod::Bronze => 10,
                            Amphipod::Copper => 100,
                            Amphipod::Desert => 1000,
                        };
                        new_state.corridor[d] = Square::Occupied(moved);
                        new_state.cost += move_cost;
                        heap.push(new_state);
                    }

                } 
            }
        }


    }
}

fn is_winning_position(bins: &[Vec<Amphipod>], depth: usize) -> bool {
    bins[0].iter().filter(|a| **a == Amphipod::Amber).count() == depth &&
    bins[1].iter().filter(|a| **a == Amphipod::Bronze).count() == depth &&
    bins[2].iter().filter(|a| **a == Amphipod::Copper).count() == depth &&
    bins[3].iter().filter(|a| **a == Amphipod::Desert).count() == depth 
    
}