use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Register {
    A,
    B,
    C,
    D,
    Unknown
}

#[derive(Debug,Copy,Clone)]

enum Value{
    Literal(i32),
    RegisterValue(Register)
}

#[derive(Debug,Clone,Copy)]
enum AssemBunnyInstruction {
    Copy(Value, Value),
    Decrement(Value),
    Increment(Value),
    JumpIfNotZero(Value, Value),
    Toggle(Value),

}

fn main() {
    let mut instructions = read_instructions("./input.txt");
    println!("{:?}", run_program(&mut instructions));
}

fn read_instructions(filename: &str) -> Vec<AssemBunnyInstruction> {
    let raw_data = std::fs::read_to_string(filename).unwrap();
    let mut program: Vec<AssemBunnyInstruction> = Vec::new();
    for line in raw_data.split('\n') {
        let words = line.split_ascii_whitespace().collect::<Vec<_>>();
        let instruction = match words[0] {
            "cpy" => {
                AssemBunnyInstruction::Copy(str_to_value(words[1]), str_to_value(words[2]))
            },
            "inc" => {
                AssemBunnyInstruction::Increment(str_to_value(words[1]))
            },
            "dec" => {
                AssemBunnyInstruction::Decrement(str_to_value(words[1]))
            },
            "jnz" => {
                AssemBunnyInstruction::JumpIfNotZero(str_to_value(words[1]), str_to_value(words[2]))
            },
            "tgl" => {
                AssemBunnyInstruction::Toggle(str_to_value(words[1]))
            }
            _ => unimplemented!()
        };
        program.push(instruction);
    }
    program
}

fn str_to_register(w: &str) -> Register {
    match w {
        "a" => Ok(Register::A),
        "b" => Ok(Register::B),
        "c" => Ok(Register::C),
        "d" => Ok(Register::D),
        _ => Err(Register::Unknown)
    }.unwrap()
}

fn str_to_value(w: &str) -> Value {
    match w.parse::<i32>() {
        Ok(n) => Value::Literal(n),
        Err(_) => Value::RegisterValue(str_to_register(w))
    }
}

fn run_program(program: &mut Vec<AssemBunnyInstruction>) -> HashMap<Register, i32> {
    let mut registers: HashMap<Register,i32> = HashMap::new();
    registers.insert(Register::A, 12);

    let mut program_counter:i32 = 0;

    while (program_counter as usize) < program.len() {

        match program.get(program_counter as usize).unwrap() {
            AssemBunnyInstruction::Copy(src, dst) => {
                match (src, dst) {
                    (Value::Literal(v), Value::RegisterValue(r)) => {
                        registers.insert(*r, *v);
                    },
                    (Value::RegisterValue(src), Value::RegisterValue(dst)) => {
                        registers.insert( *dst, *registers.get(src).unwrap_or(&0));
                    },
                    (_, _) => {}

                }
                program_counter += 1;
            },
            AssemBunnyInstruction::Increment(reg) => {
                // it's a register.
                if let Value::RegisterValue(reg) = reg {
                    registers.insert(*reg, registers.get(reg).unwrap_or(&0) + 1);
                    program_counter += 1;
                }
            },
            AssemBunnyInstruction::Decrement(reg) => {
                if let Value::RegisterValue(reg) = reg {
                    registers.insert(*reg, registers.get(reg).unwrap_or(&0) - 1);
                    program_counter += 1;
                }
            },
            AssemBunnyInstruction::JumpIfNotZero(value, increment) => {
                program_counter += if match value {
                    Value::Literal(v) => *v,
                    Value::RegisterValue(r) => *registers.get(r).unwrap_or(&0)
                } != 0 {
                    match increment {
                        Value::Literal(v) => *v,
                        Value::RegisterValue(r) => *registers.get(r).unwrap_or(&0)
                    }
                } else {
                    1
                }
            },
            AssemBunnyInstruction::Toggle(value) => {
                let offset = match value {
                    Value::Literal(v) => *v,
                    Value::RegisterValue(r) => *registers.get(&r).unwrap_or(&0)
                };
                let location_to_flip = program_counter + offset;
                if location_to_flip >= 0 && location_to_flip < program.len()  as i32 {
                    program[location_to_flip as usize] = match program.get(location_to_flip as usize).unwrap() {
                        AssemBunnyInstruction::Increment(x) => AssemBunnyInstruction::Decrement(*x),
                        AssemBunnyInstruction::Decrement(x) => AssemBunnyInstruction::Increment(*x),
                        AssemBunnyInstruction::Toggle(x) => AssemBunnyInstruction::Increment(*x),
                        AssemBunnyInstruction::JumpIfNotZero(x,y) => AssemBunnyInstruction::Copy(*x,*y),
                        AssemBunnyInstruction::Copy(x, y) => AssemBunnyInstruction::JumpIfNotZero(*x,*y)
                    };
                }
                program_counter += 1;
            }
        }
    }

    registers
}

