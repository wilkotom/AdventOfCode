use std::collections::{HashSet, VecDeque};
use std::hash::Hash;

#[derive(Hash, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Object {
    Generator(String),
    Chip(String),
}


fn main() {
    let objects = set_up_floors(String::from("./input.txt"));
    println!("Solution: {}", find_solution(objects));
}

fn set_up_floors(filename: String) -> Vec<Vec<Object>>{
    let description = std::fs::read_to_string(filename).unwrap();
    let mut floors: Vec<Vec<Object>> = Vec::new();
    for line in description.split("\n") {
        let mut floor: Vec<Object> = Vec::new();
        let tokens = line.split(" a ").collect::<Vec<_>>();
        for token in &tokens[1..] {
            let object = token.split(" ").next().unwrap();
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
    let mut seen_states: HashSet<(Vec<(usize,usize)>, usize)> = HashSet::new();
    seen_states.insert((gen_hash(&starting_state.clone()),0));

    current_moves.push_back((0, starting_state, 0));
    while !current_moves.is_empty() {

        let board = current_moves.pop_front().unwrap();
        let mut next_moves: VecDeque<(usize, Vec<Vec<Object>>, usize)> = VecDeque::new();
        let floor = board.0;
        let state= board.1;
        let moves = board.2;
        if is_winning_state(&state){
            return moves;
        }

        // Can get 1 item or 2
        for item in state[floor].clone().iter() {
            // If we're not at the bottom floor *and* all the floors floor beneath are is not empty, we can take 1 or 2 items item downward
            let mut next_state = state.clone();
            if floor > 0  && floor > first_occupied_floor(&state) { 
                next_state[floor-1].push(item.clone());
                next_state[floor] = next_state[floor].iter().filter(|x| x != &item).cloned().collect();
                next_state[floor-1].sort();
                next_state[floor].sort();
                // push a state with 1 item moved down
                if is_valid_state(&next_state){
                    let state_hash = gen_hash(&next_state);
                    if !seen_states.contains(&(state_hash.clone(), floor-1)) {
                        seen_states.insert((state_hash, floor -1));
                        next_moves.push_back((floor-1, next_state.clone(), moves+1));
                    }
                } 
                for item in next_state[floor].iter() {
                    let mut next_state = next_state.clone();
                    next_state[floor-1].push(item.clone());
                    next_state[floor] = next_state[floor].iter().filter(|x| x != &item).cloned().collect();
                    next_state[floor-1].sort();
                    next_state[floor].sort();
                    if is_valid_state(&next_state){
                        let state_hash = gen_hash(&next_state);
                        if !seen_states.contains(&(state_hash.clone(), floor-1)) {
                            seen_states.insert((state_hash, floor-1));
                                next_moves.push_back((floor-1, next_state, moves+1));
                            }
                    }  
                }

            }
            if floor < state.len() -1 { // don't try to go above the top floor
                let mut next_state = state.clone();
                next_state[floor+1].push(item.clone());
                next_state[floor] = next_state[floor].iter().filter(|x| x != &item).cloned().collect();
                next_state[floor+1].sort();
                next_state[floor].sort();
                // push a state with 1 item moved up
                println!("{}", next_state[floor].len());
                if is_valid_state(&next_state) {
                    let state_hash = gen_hash(&next_state);
                    if !seen_states.contains(&(state_hash.clone(), floor+1)) {
                        seen_states.insert((state_hash, floor+1));
                        next_moves.push_back((floor+1, next_state.clone(), moves+1));
                    }
                } 
                for item in next_state[floor].iter() {
                    let mut next_state = next_state.clone();
                    next_state[floor+1].push(item.clone());
                    next_state[floor] = next_state[floor].iter().filter(|x| x != &item).cloned().collect();
                    next_state[floor+1].sort();
                    next_state[floor].sort();
                    if is_valid_state(&next_state){
                        let state_hash = gen_hash(&next_state);
                        if !seen_states.contains(&(state_hash.clone(), floor+1)) {
                            seen_states.insert((state_hash, floor+1));
                            next_moves.push_back((floor+1, next_state, moves+1));
                        }
                    }  
                }
            }
        }
        current_moves.append(&mut next_moves);
    }
    0
}

fn is_valid_state(state: &[Vec<Object>]) -> bool {
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
            if !generators.is_empty() && !generators.contains(&chip) {
                return false;
            }
        }
    }

    true
}

fn is_winning_state(state: &[Vec<Object>]) -> bool {
    for floor in &state[0..state.len()-1] {
        if !floor.is_empty() {
            return false;
        }
    }
    true
}

fn gen_hash(state: &Vec<Vec<Object>>) -> Vec<(usize, usize)> {
    // Create a hash of each valid state consistng of pairs of valies for each fl90oor
    // counts of (chips, generators)
    // Since it doesn't matter which 
    let mut result: Vec<(usize, usize)> = Vec::new();
    for floor in state {
        let mut chips = 0;
        let mut generators = 0;
        for item in floor {
            match item {
                Object::Generator(_) => generators +=1,
                Object::Chip(_) => chips +=1
            }
        }
        result.push((chips, generators));
    }
    result
}

fn first_occupied_floor(state: &[Vec<Object>]) -> usize {
    for (i, n) in state.iter().enumerate() {
        if !n.is_empty() {
            return i;
        }
    }
    state.len()
}