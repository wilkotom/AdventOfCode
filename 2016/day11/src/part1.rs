use std::collections::{HashSet, VecDeque};
use std::hash::Hash;

#[derive(Hash, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Object {
    Generator(String),
    Chip(String),
}


fn main() {
    let objects = set_up_floors(String::from("./input.txt"));
    println!("{:?}", objects);
    println!("Solution Part 1: {}", find_solution(objects));
}

fn set_up_floors(filename: String) -> Vec<Vec<Object>>{
    let description = std::fs::read_to_string(filename).unwrap();
    let mut floors: Vec<Vec<Object>> = Vec::new();
    for line in description.split("\n") {
        let mut floor: Vec<Object> = Vec::new();
        let tokens = line.split(" a ").collect::<Vec<_>>();
        for token in &tokens[1..] {
            let object = token.split(" ").next().unwrap();
            // println!("{}", object);
            let object = String::from(object);
            if ! object.contains("-") {
                floor.push(Object::Generator(object));
            } else {
                let chip_name = String::from(object.split("-").next().unwrap());
                floor.push(Object::Chip(chip_name));
            }
        }
        floor.sort();
        floors.push(floor);
    }
    floors
}

fn find_solution(starting_state: Vec<Vec<Object>>) -> usize {
    let mut current_moves: VecDeque<(usize, Vec<Vec<Object>>, usize)> = VecDeque::new();
    let mut seen_states: HashSet<(Vec<Vec<Object>>, usize)> = HashSet::new();
    seen_states.insert((starting_state.clone(),0));

    current_moves.push_back((0, starting_state, 0));
    println!("Start");
    while current_moves.len() > 0 {

        let board = current_moves.pop_front().unwrap();
        let mut next_moves: VecDeque<(usize, Vec<Vec<Object>>, usize)> = VecDeque::new();
        let floor = board.0;
        let state= board.1;
        let moves = board.2;
        println!("Move: {}", moves);
        if is_winning_state(&state){
            println!("Winning state: {:?}", state);
            return moves;
        }
        // println!("Floor {}, can move {:?}", floor, state[floor]);
        // println!("Move {}", moves);
        // println!("Current state: {:?}", state);
        // Can get 1 item or 2
        for item in state[floor].clone().iter() {
            // println!("Considering moving {:?}", item);
            // If we're not at the bottom floor *and* the floor beneath us is not empty, we can take 1 or 2 items item downward
            let mut next_state = state.clone();
            if floor > 0  && floor > first_occupied_floor(&state) { 
                next_state[floor-1].push(item.clone());
                next_state[floor] = next_state[floor].iter().filter(|x| x != &item).map(|x| x.clone()).collect();
                next_state[floor-1].sort();
                next_state[floor].sort();
                // push a state with 1 item moved down
                if is_valid_state(&next_state){

                    if !seen_states.contains(&(next_state.clone(), floor-1)) {
                        seen_states.insert((next_state.clone(), floor -1));
                        next_moves.push_back((floor-1, next_state.clone(), moves+1));
                    }
                } else {
                    // println!("Seen state {:?} before", next_state);
                    // println!("Seen states: {:?}", seen_states);
                }
                for item in next_state[floor].iter() {
                    let mut next_state = next_state.clone();
                    next_state[floor-1].push(item.clone());
                    next_state[floor] = next_state[floor].iter().filter(|x| x != &item).map(|x| x.clone()).collect();
                    next_state[floor-1].sort();
                    next_state[floor].sort();
                    if is_valid_state(&next_state){
                        if !seen_states.contains(&(next_state.clone(), floor-1)) {
                            seen_states.insert((next_state.clone(), floor-1));
                            next_moves.push_back((floor-1, next_state, moves+1));
                        }
                    }  else {
                        // println!("Seen state {:?} before", next_state);
                        // println!("Seen states: {:?}", seen_states);
                    }
                }

            }
            if floor < state.len() -1 { // don't try to go above the top floor
                let mut next_state = state.clone();
                next_state[floor+1].push(item.clone());
                next_state[floor] = next_state[floor].iter().filter(|x| x != &item).map(|x| x.clone()).collect();
                next_state[floor+1].sort();
                next_state[floor].sort();
                // push a state with 1 item moved up
                if is_valid_state(&next_state){
                    if !seen_states.contains(&(next_state.clone(), floor+1)) {
                        seen_states.insert((next_state.clone(), floor+1));
                        next_moves.push_back((floor+1, next_state.clone(), moves+1));
                    }
                }  else {
                    // println!("Seen state {:?} before", next_state);
                    // println!("Seen states: {:?}", seen_states);
                }
                for item in next_state[floor].iter() {
                    let mut next_state = next_state.clone();
                    next_state[floor+1].push(item.clone());
                    next_state[floor] = next_state[floor].iter().filter(|x| x != &item).map(|x| x.clone()).collect();
                    next_state[floor+1].sort();
                    next_state[floor].sort();
                    if is_valid_state(&next_state){
                        if !seen_states.contains(&(next_state.clone(), floor+1)) {
                            seen_states.insert((next_state.clone(), floor+1));
                            next_moves.push_back((floor+1, next_state, moves+1));
                        }
                    }  else {
                        // println!("Seen state {:?} before", next_state);
                        // println!("Seen states: {:?}", seen_states);
                    }
                }
            }
        }
        // for move_ in &next_moves{
        //     println!("Possible move: {:?}", move_);
        // }
        // println!("{:?}", next_moves);
        current_moves.append(&mut next_moves);
    }
    0
}

fn is_valid_state(state: &Vec<Vec<Object>>) -> bool {
    // print!("Validating state: {:?}", state);
    for floor in state.iter(){
        let mut generators: HashSet<String> = HashSet::new();
        let mut chips: HashSet<String> = HashSet::new();
        for object in floor{
            match object {
                Object::Chip(x) => {chips.insert(x.clone());},
                Object::Generator(x) => {generators.insert(x.clone());}
            }
        } 
        for chip in chips{
            if generators.len() > 0 && !generators.contains(&chip) {
                // println!(" - INVALID");
                return false;
            }
        }
    }
    // println!(" - OK");

    true
}

fn is_winning_state(state: &Vec<Vec<Object>>) -> bool {
    for floor in &state[0..state.len()-1] {
        // println!("{:?}", floor);
        if floor.len() > 0 {
            return false;
        }
    }
    true
}

fn first_occupied_floor(state: &Vec<Vec<Object>>) -> usize {
    for (i, n) in state.iter().enumerate() {
        if n.len() > 0 {
            return i;
        }
    }
    state.len()
}