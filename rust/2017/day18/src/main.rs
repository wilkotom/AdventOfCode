use std::collections::HashMap;

#[derive(Debug,Clone)]
struct Instruction {
    inst: DuetInstruction,
    a: Argument,
    b: Argument
}

#[derive(Debug,Copy,Clone)]
enum DuetInstruction {
    Send,
    Set,
    Add,
    Multiply,
    Modulus,
    Receive,
    JumpGreaterThanZero
}

#[derive(Debug,Clone)]
enum Argument {
    Register(String),
    Literal(i64),
    Missing
}

fn main() {
    let data = std::fs::read_to_string("./input.txt").unwrap();
    let program = read_program(&data);
    let mut program_a_registers: HashMap<String, i64> = HashMap::new();
    let mut program_b_registers: HashMap<String, i64> = HashMap::new();

    let mut program_a_input_buffer = Vec::new();
    let mut program_b_input_buffer = Vec::new();
    let mut mpc_a = 0;
    let mut mpc_b = 0;
    program_a_registers.insert(String::from("p"), 0);
    program_b_registers.insert(String::from("p"), 1);

    mpc_a = run_program(&program, &mut program_a_registers, mpc_a, &mut program_a_input_buffer, &mut program_b_input_buffer );
    let mut prog_b_out = 0;

    while ! (program_a_input_buffer.is_empty() &&  program_b_input_buffer.is_empty()) {
        mpc_a = run_program(&program, &mut program_a_registers, mpc_a, &mut program_a_input_buffer, &mut program_b_input_buffer );
        mpc_b = run_program(&program, &mut program_b_registers, mpc_b, &mut program_b_input_buffer, &mut program_a_input_buffer );
        prog_b_out += program_a_input_buffer.len();

    }
    println!("{} ", prog_b_out);




}

fn run_program(program: &[Instruction], 
               registers: &mut HashMap<String, i64>, 
               pc: i64, 
               input_buffer: &mut Vec<i64>,
               output_buffer:&mut Vec<i64> ) -> i64 {
    let mut pc = pc;
    // let mut registers: HashMap<String, i64> = HashMap::new();
    while (pc as usize) < program.len() {
        match &program[pc as usize] {
            Instruction{inst: DuetInstruction::Send, a, b: _  } => {
                match a {
                    Argument::Register(reg) => {output_buffer.push(registers[reg])},
                    Argument::Literal(n) => {output_buffer.push(*n)}
                    Argument::Missing => unimplemented!(),
                };
                pc +=1;
            },

            Instruction{inst: DuetInstruction::Set, a, b  } => {
                let reg = match a {
                    Argument::Register(r) => r,
                    _ => unimplemented!(),
                }.to_owned();

                let value = *match b {
                    Argument::Register(r) => registers.get(r).unwrap_or(&0),
                    Argument::Literal(v) => v,
                    Argument::Missing => unimplemented!(),
                };
                registers.insert(reg,value);
                pc +=1;
            },

            Instruction{inst: DuetInstruction::Add, a, b  } => {
                let reg = match a {
                    Argument::Register(r) => r,
                    _ => unimplemented!(),
                }.to_owned();

                let value = *match b {
                    Argument::Register(r) => registers.get(r).unwrap_or(&0),
                    Argument::Literal(v) => v,
                    Argument::Missing => unimplemented!(),
                };
                let a = *registers.get(&reg).unwrap_or(&0);
                registers.insert(reg,a + value);
                pc +=1;
            },

            Instruction{inst: DuetInstruction::Multiply, a, b  } => {
                let reg = match a {
                    Argument::Register(r) => r.to_owned(),
                    _ => unimplemented!(),
                }.to_owned();

                let value = *match b {
                    Argument::Register(r) => registers.get(r).unwrap_or(&0),
                    Argument::Literal(v) => v,
                    Argument::Missing => unimplemented!(),
                };
                let a = *registers.get(&reg).unwrap_or(&0);

                registers.insert(reg,a * value);
                pc +=1;
            },

            Instruction{inst: DuetInstruction::Modulus, a, b  } => {
                let reg = match a {
                    Argument::Register(r) => r,
                    _ => unimplemented!(),
                }.to_owned();

                let value = *match b {
                    Argument::Register(r) => registers.get(r).unwrap_or(&0),
                    Argument::Literal(v) => v,
                    Argument::Missing => unimplemented!(),
                };
                let a = *registers.get(&reg).unwrap_or(&0);
                registers.insert(reg,a % value);

                pc +=1;
            },

            Instruction{inst: DuetInstruction::Receive, a, b: _  } => {
                let register = match a {
                    Argument::Register(r) => r,
                    Argument::Literal(_) => unimplemented!(),
                    Argument::Missing => unimplemented!(),
                }.to_owned();
                if input_buffer.is_empty() {
                    return pc;
                } else{
                    registers.insert(register, input_buffer.remove(0));
                }

                pc +=1;

            },

            Instruction{inst: DuetInstruction::JumpGreaterThanZero, a, b  } => {
                let a = *match a {
                    Argument::Register(r) => registers.get(r).unwrap_or(&0),
                    Argument::Literal(v) => v,
                    Argument::Missing => unimplemented!(),
                };
                let b = *match b {
                    Argument::Register(r) => registers.get(r).unwrap_or(&0),
                    Argument::Literal(v) => v,
                    Argument::Missing => unimplemented!(),
                };
                if a > 0 {
                    pc += b;
                } else {
                    pc +=1;
                }

            }
        }
    }
    unreachable!();
}

fn read_program(data: &str) -> Vec<Instruction> {
    let mut program: Vec<Instruction> = Vec::new();
    for line in data.split('\n') {
        let mut tokens = line.split_ascii_whitespace();
        match tokens.next() {
            Some("snd") => {
                let a = tokens.next().unwrap();
                let first = if let Ok(n) = a.parse::<i64>() {
                    Argument::Literal(n)
                } else {
                    Argument::Register(a.to_owned())
                };
                program.push(Instruction{
                    inst: DuetInstruction::Send,
                    a: first,
                    b: Argument::Missing});
            },
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
                    inst: DuetInstruction::Set,
                    a: Argument::Register(a),
                    b: second});
            },
            Some("add") => {
                let a = tokens.next().unwrap().to_owned();
                let b = tokens.next().unwrap();

                let second;
                if let Ok(n) = b.parse::<i64>() {
                    second= Argument::Literal(n);
                } else {
                    second = Argument::Register(b.to_owned())
                }
                program.push(Instruction{
                    inst: DuetInstruction::Add,
                    a: Argument::Register(a),
                    b: second});

                // let a = tokens.next().unwrap();
                // let b = tokens.next().unwrap();
                // let a_val = a.parse::<i64>().unwrap_or(*registers.get(&a).unwrap_or(&0));
                // let b_val = b.parse::<i64>().unwrap_or(*registers.get(&b).unwrap_or(&0));
                // registers.insert(a, a_val + b_val);

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
                    inst: DuetInstruction::Multiply,
                    a: Argument::Register(a),
                    b: second});

            },
            Some("mod") => {
                let a = tokens.next().unwrap().to_owned();
                let b = tokens.next().unwrap();
                let second;
                if let Ok(n) = b.parse::<i64>() {
                    second= Argument::Literal(n);
                } else {
                    second = Argument::Register(b.to_owned())
                }
                program.push(Instruction{
                    inst: DuetInstruction::Modulus,
                    a: Argument::Register(a),
                    b: second});

            },
            Some("rcv") => {
                let a = tokens.next().unwrap();
                let first;
                if let Ok(n) = a.parse::<i64>() {
                    first = Argument::Literal(n)
                } else {
                    first = Argument::Register(a.to_owned())
                }
                program.push(Instruction{
                    inst: DuetInstruction::Receive,
                    a: first,
                    b: Argument::Missing});
                // sounds.push(a.parse::<i64>().unwrap_or(*registers.get(&a).unwrap()));
            },
            Some("jgz") => {
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
                    inst: DuetInstruction::JumpGreaterThanZero,
                    a: first,
                    b: second});
                },


            _ => todo!(),
        }
    }
    program
}
