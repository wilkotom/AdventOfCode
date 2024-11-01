use std::fs;
use std::io::Error;
use std::collections::HashSet;

enum Instruction {
    JMP,
    NOP,
    ACC
}

impl Clone for Instruction{
    fn clone(&self) -> Instruction {
        match self {
            Instruction::JMP => Instruction::JMP,
            Instruction::NOP => Instruction::NOP,
            Instruction::ACC => Instruction::ACC
        }
    }
}

#[derive(Clone)]
struct ProgramLine {
    instruction: Instruction,
    value: i32
}

fn main() -> Result<(), Error> {
    let program = read_program("../input.txt".to_string())?;
    println!("Part 1 answer: {}",run_program(program.clone()).0);
    let mut suspects: Vec<usize> = Vec::new();
    for i in 0..program.len() {
        match program[i] {
            ProgramLine {instruction: Instruction::NOP, .. } | 
            ProgramLine {instruction: Instruction::JMP, .. } 
            => {
                suspects.push(i);
            }
            _ => {}
        }
    }
    for suspect in suspects.iter() {
        let mut new_prog = program.clone();
        new_prog[*suspect] = match new_prog[*suspect] {
            ProgramLine {instruction: Instruction::JMP, value } => 
                ProgramLine {instruction: Instruction::NOP, value },
            ProgramLine {instruction: Instruction::NOP, value } =>
                ProgramLine {instruction: Instruction::JMP, value } ,
            _ => new_prog[*suspect].clone()
        };
        let result = run_program(new_prog);
        if result.1 == true {
            println!("Part 2 answer: {}", result.0);
        }
    }
    Ok(())
}

fn read_program(filename: String) -> Result<Vec<ProgramLine>, Error> {
    let raw_program = fs::read_to_string(filename)?;
    let mut program: Vec<ProgramLine> = Vec::new();
    let lines: Vec<_> = raw_program.split("\n").collect();
    for line in lines {
        let tokens: Vec<_> = line.split(" ").collect();
        let value: i32 = tokens[1].parse().unwrap();
        let instruction = match tokens[0] {
            "acc" => Instruction::ACC,
            "jmp" => Instruction::JMP,
            "NOP" => Instruction::NOP,
            _ => Instruction::NOP
        };
        program.push(ProgramLine{instruction, value});
    }
    Ok(program)
}

fn run_program(program: Vec<ProgramLine>) -> (i32, bool){
    let mut accumulator = 0;
    let mut instruction_pointer:i32  = 0;
    let mut visited: HashSet<i32> = HashSet::new();
    while instruction_pointer < program.len() as i32 && !visited.contains(&instruction_pointer) {
        visited.insert(instruction_pointer);
        match program[instruction_pointer as usize] {
           ProgramLine {
            instruction: Instruction::ACC, 
            value } => { 
                accumulator += value;
                instruction_pointer += 1;
            },
            ProgramLine {
                instruction: Instruction::JMP, 
                value } => {
                instruction_pointer += value;
            },
            _ => {
                instruction_pointer +=1;
            }
        }
    }

    (accumulator, (instruction_pointer as usize) == program.len())
}