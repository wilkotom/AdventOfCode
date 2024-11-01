use std::collections::{HashMap, HashSet};

fn main() {
    let data = std::fs::read_to_string("./input.txt").unwrap();
    let mut sections = data.split("\n\n\n\n");
    let results = sections.next().unwrap();
    let opcode_mappings = identify_opcodes(results);
    let program = sections.next().unwrap();
    println!("Part 2: {}", run_program(opcode_mappings, program));
}

fn run_program(opcode_mappings:  HashMap<i32, String>, program: &str) -> i32 {
    let mut registers = vec![0;4];
    for line in program.split('\n').map(|l| l.split_ascii_whitespace().map(|v| v.parse::<i32>().unwrap()).collect::<Vec<_>>()) {
        execute(&opcode_mappings[&line[0]], &line[1..], &mut registers);
    }
    registers[0]
}

fn identify_opcodes(input_data: &str) -> HashMap<i32, String> {

    let opcodes = HashSet::from(["addr", "addi", "mulr", "muli", "banr", "bani", "borr", "bori", "setr", "seti", "gtir", "gtri", "gtrr", "eqir", "eqri", "eqrr" ]);
    let mut mappings = vec![opcodes.clone(); 16];
    let mut three_or_more = 0;
    for result in input_data.split("\n\n") {
        let mut lines = result.split('\n');
        let line = lines.next().unwrap();
        let before = line[9..line.len()-1].split(", ").map(|v| v.parse::<i32>().unwrap()).collect::<Vec<_>>();
        let instr = lines.next().unwrap().split_ascii_whitespace().map(|v| v.parse::<i32>().unwrap()).collect::<Vec<_>>();
        let line = lines.next().unwrap();
        let after = line[9..line.len()-1].split(", ").map(|v| v.parse::<i32>().unwrap()).collect::<Vec<_>>();
        let mut counter = 0;
        for possible_instr in &opcodes {
            let mut registers = before.clone();
            execute(possible_instr, &instr[1..], &mut registers);
            if registers != after {
                mappings.get_mut(instr[0] as usize).unwrap().remove(possible_instr);
        
            } else {
                counter += 1;
            }
        }
        if counter >=3 {
            three_or_more +=1;
        }

    }
    println!("Part 1: {}", three_or_more);
    while mappings.iter().map(|x| x.len()).max().unwrap() > 1 {
        for (i, instrs) in mappings.clone().iter().enumerate() {
            if instrs.len() == 1 {
                let instr = instrs.iter().next().unwrap();
                for (j, mapping) in mappings.iter_mut().enumerate() {
                    if i != j {
                        mapping.remove(instr.to_owned());
                    }
                }
            }
        }
    }
    mappings.iter().enumerate().map(|(k,v)| (k as i32, v.iter().next().unwrap().to_string())).collect::<HashMap<_,_>>()
}

fn execute(instr: &str, args: &[i32], registers: &mut [i32]) {

    match instr {
        "addr" => {
            registers[args[2] as usize] = registers[args[0] as usize] + registers[args[1] as usize];
        },
        "addi" => {
            registers[args[2] as usize] = registers[args[0] as usize] + args[1];
        },
        "mulr" => {
            registers[args[2] as usize] = registers[args[0] as usize] * registers[args[1] as usize];
        },
        "muli" => {
            registers[args[2] as usize] = registers[args[0] as usize] * args[1];
        },
        "banr" => {
            registers[args[2] as usize] = registers[args[0] as usize] & registers[args[1] as usize];
        },
        "bani" => {
            registers[args[2] as usize] = registers[args[0] as usize] & args[1];
        },
        "borr" => {
            registers[args[2] as usize] = registers[args[0] as usize] | registers[args[1] as usize];
        },
        "bori" => {
            registers[args[2] as usize] = registers[args[0] as usize] | args[1];
        },
        "setr" => {
            registers[args[2] as usize] = registers[args[0] as usize]; 
        },
        "seti" => {
            registers[args[2] as usize] = args[0];
        },
        "gtir" => {
            registers[args[2] as usize] = if args[0] > registers[args[1] as usize] {1} else {0};
        },
        "gtri" => {
            registers[args[2] as usize] = if registers[args[0] as usize] > args[1] {1} else {0};
        },
        "gtrr" => {
            registers[args[2] as usize] = if registers[args[0] as usize] > registers[args[1] as usize] {1} else {0};
        },
        "eqir" => {
            registers[args[2] as usize] = if args[0] == registers[args[1] as usize] {1} else {0};
        },
        "eqri" => {
            registers[args[2] as usize] = if registers[args[0] as usize] == args[1] {1} else {0};
        },
        "eqrr" => {
            registers[args[2] as usize] = if registers[args[0] as usize] == registers[args[1] as usize] {1} else {0};
        },
        _ => unimplemented!()
    }
}