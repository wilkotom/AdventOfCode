use std::{fs::read_to_string, collections::HashMap};

#[derive(Debug,Copy, Clone, Hash, Eq, PartialEq)]
enum Register{
    A,
    B
}

#[derive(Debug,Copy, Clone)]
enum Argument{
    Register(Register),
    Value(isize),
    Nothing
}

#[derive(Debug,Copy, Clone)]
enum Day23Instruction{
    Half(Argument),
    Triple(Argument),
    Increment(Argument),
    Jump(Argument),
    JumpIfEven(Argument, Argument),
    JumpIfOne(Argument, Argument)
}


fn main() {
    let program =  read_instructions("./input.txt");
    let mut registers: HashMap<Register, usize> = HashMap::new();
    registers.insert(Register::A, 0);
    registers.insert(Register::B, 0);
    println!("Part 1: {}", run_program(registers, &program));
    let mut registers: HashMap<Register, usize> = HashMap::new();
    registers.insert(Register::A, 1);
    registers.insert(Register::B, 0);
    println!("Part 2: {}", run_program(registers, &program));
}


fn read_instructions(filename: &str) -> Vec<Day23Instruction> {
    let data = read_to_string(filename).unwrap();
    let mut program = Vec::new();
    for line in data.split('\n') {
        let mut words = line.split_ascii_whitespace();
        let instr = words.next();
        let first_arg = match words.next() {
            Some("a") | Some("a,") => Argument::Register(Register::A),
            Some("b") | Some("b,") => Argument::Register(Register::B),
            Some(n) => match n.parse::<isize>() {
                Ok(v) => Argument::Value(v),
                _ => unimplemented!()
            },
            None => unimplemented!()
        };
        let second_arg = match words.next() {
            Some(n) => match n.parse::<isize>() {
                Ok(v) => Argument::Value(v),
                _ => unimplemented!()
            },
            None => {Argument::Nothing}
        };
        program.push(match instr {
            Some("hlf") => {Day23Instruction::Half(first_arg)},
            Some("tpl") => {Day23Instruction::Triple(first_arg)},
            Some("inc") => Day23Instruction::Increment(first_arg),
            Some("jmp") => Day23Instruction::Jump(first_arg),
            Some("jie") => Day23Instruction::JumpIfEven(first_arg, second_arg),
            Some("jio") => Day23Instruction::JumpIfOne(first_arg, second_arg),
            _ => unimplemented!()

        });
    }


    program
}

fn run_program(mut registers: HashMap<Register, usize>, program: &[Day23Instruction]) -> usize {
    let mut pc = 0;
    while pc < program.len() {
        match program[pc] {
            Day23Instruction::Half(Argument::Register(r)) => {
                registers.insert(r, registers.get(&r).unwrap() /2 );
                pc += 1;
            },
            Day23Instruction::Triple(Argument::Register(r)) => {
                registers.insert(r, registers.get(&r).unwrap() *3 );
                pc +=1;
            },
            Day23Instruction::Increment(Argument::Register(r)) => {
                registers.insert(r, registers.get(&r).unwrap() +1 );
                pc +=1;
            },
            Day23Instruction::Jump(Argument::Register(r)) => {
                pc += registers.get(&r).unwrap();
            },
            Day23Instruction::Jump(Argument::Value(v)) => {
                pc = (pc as isize + v) as usize;
            },
            Day23Instruction::JumpIfEven(Argument::Register(r), Argument::Value(v)) => {
                if registers.get(&r).unwrap() % 2 == 0 {
                    pc = (pc as isize + v) as usize;
                } else {
                    pc += 1;
                }
            },
            Day23Instruction::JumpIfOne(Argument::Register(r), Argument::Value(v)) => {
                if registers.get(&r).unwrap() == &1 {
                    pc = (pc as isize + v) as usize;
                } else {
                    pc += 1;
                }
            },
            _ => unimplemented!()
            }
    }
    registers[&Register::B]
}