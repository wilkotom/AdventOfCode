use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Register {
    A,
    B,
    C,
    D,
    Unknown
}

#[derive(Debug)]

enum Value{
    Literal(i32),
    RegisterValue(Register)
}

#[derive(Debug)]
enum AssemBunnyInstruction {
    Copy(Value, Register),
    Decrement(Register),
    Increment(Register),
    JumpIfNotZero(Value, Value),
    Unknown

}

fn main() {
    let instructions = read_instructions("./input.txt");
    println!("{:?}", run_program(instructions));
}

fn read_instructions(filename: &str) -> Vec<AssemBunnyInstruction> {
    let raw_data = std::fs::read_to_string(filename).unwrap();
    let mut program: Vec<AssemBunnyInstruction> = Vec::new();
    for line in raw_data.split("\n") {
        let words = line.split_ascii_whitespace().collect::<Vec<_>>();
        let instruction = match words[0] {
            "cpy" => {
                AssemBunnyInstruction::Copy(str_to_value(words[1]), str_to_register(words[2]))
            },
            "inc" => {
                AssemBunnyInstruction::Increment(str_to_register(&words[1]))
            },
            "dec" => {
                AssemBunnyInstruction::Decrement(str_to_register(&words[1]))
            },
            "jnz" => {
                AssemBunnyInstruction::JumpIfNotZero(str_to_value(words[1]), str_to_value(words[2]))
            },
            _ => AssemBunnyInstruction::Unknown
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

fn run_program(program: Vec<AssemBunnyInstruction>) -> HashMap<Register, i32> {
    let mut registers: HashMap<Register,i32> = HashMap::new();
    registers.insert(Register::C, 1);

    let mut program_counter:i32 = 0;

    while (program_counter as usize) < program.len() {
        // println!("{:?}", program.get(
        //     program_counter as usize).unwrap());
        // println!("{:?}", registers);
        match program.get(program_counter as usize).unwrap() {
            AssemBunnyInstruction::Copy(src, dst) => {
                registers.insert(*dst, match src{
                    Value::Literal(v) => *v,
                    Value::RegisterValue(r) => *registers.get(&r).unwrap_or(&0)
                });
                program_counter += 1;
            },
            AssemBunnyInstruction::Increment(reg) => {
                registers.insert(*reg, registers.get(&reg).unwrap_or(&0) + 1);
                program_counter += 1;
            },
            AssemBunnyInstruction::Decrement(reg) => {
                registers.insert(*reg, registers.get(&reg).unwrap_or(&0) - 1);
                program_counter +=1;
            },
            AssemBunnyInstruction::JumpIfNotZero(value, increment) => {
                program_counter += if match value {
                    Value::Literal(v) => *v,
                    Value::RegisterValue(r) => *registers.get(&r).unwrap_or(&0)
                } != 0 {
                    match increment {
                        Value::Literal(v) => *v,
                        Value::RegisterValue(r) => *registers.get(&r).unwrap_or(&0)
                    }
                } else {
                    1
                }
            },
            AssemBunnyInstruction::Unknown => {}
        }
    }

    registers
}

