use std::collections::HashMap;

#[derive(Debug,Clone)]
struct Instruction {
    inst: CoProcInstruction,
    a: Argument,
    b: Argument
}

#[derive(Debug,Copy,Clone)]
enum CoProcInstruction {
    Set,
    Sub,
    Multiply,
    JumpIfNotZero
}

#[derive(Debug,Clone)]
enum Argument {
    Register(String),
    Literal(i64)
}



fn main() {
    let data = std::fs::read_to_string("./input.txt").unwrap();
    let program = read_program(&data);
    run_program(&program);
}

fn run_program(program: &[Instruction])  {

    let mut pc = 0;
    let mut registers: HashMap<String, i64> = HashMap::new();
    registers.insert("a".to_string(), 1);
    let mut mul_counter = 0;
    // let mut registers: HashMap<String, i64> = HashMap::new();
    while (pc as usize) < program.len() {
        match &program[pc as usize] {

            Instruction{inst: CoProcInstruction::Set, a, b  } => {
                let reg = match a {
                    Argument::Register(r) => r,
                    _ => unimplemented!(),
                }.to_owned();

                let value = *match b {
                    Argument::Register(r) => registers.get(r).unwrap_or(&0),
                    Argument::Literal(v) => v,
                };
                registers.insert(reg,value);
                pc +=1;
            },

            Instruction{inst: CoProcInstruction::Sub, a, b  } => {
                let reg = match a {
                    Argument::Register(r) => r,
                    _ => unimplemented!(),
                }.to_owned();

                let value = *match b {
                    Argument::Register(r) => registers.get(r).unwrap_or(&0),
                    Argument::Literal(v) => v,
                };
                let a = *registers.get(&reg).unwrap_or(&0);
                registers.insert(reg,a - value);
                pc +=1;
            },

            Instruction{inst: CoProcInstruction::Multiply, a, b  } => {
                let reg = match a {
                    Argument::Register(r) => r.to_owned(),
                    _ => unimplemented!(),
                }.to_owned();

                let value = *match b {
                    Argument::Register(r) => registers.get(r).unwrap_or(&0),
                    Argument::Literal(v) => v,
                };
                let a = *registers.get(&reg).unwrap_or(&0);

                registers.insert(reg,a * value);
                pc +=1;
                mul_counter +=1;
            },

            Instruction{inst: CoProcInstruction::JumpIfNotZero, a, b  } => {
                let a = *match a {
                    Argument::Register(r) => registers.get(r).unwrap_or(&0),
                    Argument::Literal(v) => v,
                };
                let b = *match b {
                    Argument::Register(r) => registers.get(r).unwrap_or(&0),
                    Argument::Literal(v) => v,
                };
                if a != 0 {
                    pc += b;
                } else {
                    pc +=1;
                }

            }
        }
    }
    println!("Registers: {}", mul_counter);
}

fn read_program(data: &str) -> Vec<Instruction> {
    let mut program: Vec<Instruction> = Vec::new();
    for line in data.split('\n') {
        let mut tokens = line.split_ascii_whitespace();
        match tokens.next() {
            Some("set") => {
                let a = tokens.next().unwrap().to_owned();
                let b = tokens.next().unwrap();

                let second;
                if let Ok(n) = b.parse::<i64>() {
                    second= Argument::Literal(n);
                } else {
                    second = Argument::Register(b.to_owned())
                }
                program.push(Instruction{
                    inst: CoProcInstruction::Set,
                    a: Argument::Register(a),
                    b: second});
            },
            Some("sub") => {
                let a = tokens.next().unwrap().to_owned();
                let b = tokens.next().unwrap();

                let second;
                if let Ok(n) = b.parse::<i64>() {
                    second= Argument::Literal(n);
                } else {
                    second = Argument::Register(b.to_owned())
                }
                program.push(Instruction{
                    inst: CoProcInstruction::Sub,
                    a: Argument::Register(a),
                    b: second});

            },
            Some("mul") => {
                let a = tokens.next().unwrap().to_owned();
                let b = tokens.next().unwrap();
                let second;
                if let Ok(n) = b.parse::<i64>() {
                    second= Argument::Literal(n);
                } else {
                    second = Argument::Register(b.to_owned())
                }
                program.push(Instruction{
                    inst: CoProcInstruction::Multiply,
                    a: Argument::Register(a),
                    b: second});

            },

            Some("jnz") => {
                let a = tokens.next().unwrap().to_owned();
                let b = tokens.next().unwrap().to_owned();
                let first = if let Ok(n) = a.parse::<i64>() {
                     Argument::Literal(n)
                } else {
                    Argument::Register(a.to_owned())
                };
                let second = if let Ok(n) = b.parse::<i64>() {
                    Argument::Literal(n)
                } else {
                    Argument::Register(a.to_owned())
                };
                program.push(Instruction{
                    inst: CoProcInstruction::JumpIfNotZero,
                    a: first,
                    b: second});
                },


            _ => todo!(),
        }
    }
    program
}
