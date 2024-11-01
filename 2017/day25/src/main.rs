use std::collections::{HashMap, HashSet};

#[derive(Debug,Clone, Copy)]
struct Instruction {
    write: bool,
    direction: i32,
    next_state: char
}

fn main() {
    let data = std::fs::read_to_string("./input.txt").unwrap();
    let mut sections = data.split("\n\n");
    let mut truths: HashMap<char,Instruction> = HashMap::new();
    let mut falsehoods: HashMap<char,Instruction> = HashMap::new();
    let mut tape: HashSet<i32>= HashSet::new();
    let mut position = 0;

    let preamble = sections.next().unwrap().split_ascii_whitespace().collect::<Vec<_>>();
    let mut state = preamble[3].chars().next().unwrap();
    let iterations = preamble[9].parse::<i32>().unwrap();
    println!("{} {}", state, iterations);

    for section in sections {
        let details = section.split_ascii_whitespace().collect::<Vec<_>>();
        let state = details[2].chars().next().unwrap();
        let write = details[13] == "1.";
        let direction = if details[20] == "right." {1} else {-1};
        let next_state = details[25].chars().next().unwrap();
        falsehoods.insert(state, Instruction{write, direction, next_state});

        let write = details[36] == "1.";
        let direction = if details[43] == "right." {1} else {-1};
        let next_state = details[48].chars().next().unwrap();
        truths.insert(state, Instruction{write, direction, next_state});

    }

    for _ in 0..iterations {
        if tape.contains(&position) {
            if !truths[&state].write {
                tape.remove(&position);
            }
            position += truths[&state].direction;
            state = truths[&state].next_state;
        } else {
            if falsehoods[&state].write {
                tape.insert(position);
            }
            position += falsehoods[&state].direction;
            state = falsehoods[&state].next_state;
        }
    }

    println!("{:?}", tape.len());
}
