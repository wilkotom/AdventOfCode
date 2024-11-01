use std::{fs::read_to_string, collections::HashMap};

#[derive(Debug,Clone, PartialEq, Eq, Hash)]
enum Value {
    Literal(u16),
    Register(String),
}

#[derive(Debug,Clone, PartialEq, Eq, Hash)]
enum Instruction{
    And(Value, Value),
    Or(Value, Value),
    Lshift(Value, Value),
    Rshift(Value, Value),
    Not(Value),
    NoOp(Value)
}

fn main() {
    let instructions = read_instructions("./input.txt");
    let mut result_cache: HashMap<String, u16> = HashMap::new();
    println!("{:?}", resolve("a".to_owned(), &instructions, &mut result_cache));
}

fn resolve(wire: String, instructions: &HashMap<String, Instruction>, result_cache: &mut HashMap<String, u16>) -> u16{
    if result_cache.contains_key(&wire) {
        *result_cache.get(&wire).unwrap()
    } else {
        let res = match instructions.get(&wire) {
            Some(Instruction::NoOp(i)) => {
                match i {
                    Value::Literal(v) => *v,
                    Value::Register(r) => resolve(r.clone(), instructions, result_cache),
                }
            }
            Some(Instruction::Not(i)) => {
                ! match i {
                    Value::Literal(v) => *v,
                    Value::Register(r) => resolve(r.clone(), instructions, result_cache),
                }
            }
            Some(Instruction::And(ra,la, )) => {
                (match ra {
                    Value::Literal(v) => *v,
                    Value::Register(r) => resolve(r.clone(), instructions, result_cache),
                }) & (match la {
                    Value::Literal(v) => *v,
                    Value::Register(r) => resolve(r.clone(), instructions, result_cache),
                })
            }
            Some(Instruction::Or(ra,la, )) => {
                (match ra {
                    Value::Literal(v) => *v,
                    Value::Register(r) => resolve(r.clone(), instructions, result_cache),
                }) | (match la {
                    Value::Literal(v) => *v,
                    Value::Register(r) => resolve(r.clone(), instructions, result_cache),
                })
            }
            Some(Instruction::Rshift(ra,la, )) => {
                (match ra {
                    Value::Literal(v) => *v,
                    Value::Register(r) => resolve(r.clone(), instructions, result_cache),
                }) >> (match la {
                    Value::Literal(v) => *v,
                    Value::Register(r) => resolve(r.clone(), instructions, result_cache),
                })
            }
            Some(Instruction::Lshift(ra,la, )) => {
                (match ra {
                    Value::Literal(v) => *v,
                    Value::Register(r) => resolve(r.clone(), instructions, result_cache),
                }) << (match la {
                    Value::Literal(v) => *v,
                    Value::Register(r) => resolve(r.clone(), instructions, result_cache),
                })
            }
            None => unimplemented!(),
        };
        result_cache.insert(wire,res);
        res
    }
}


fn read_instructions(filename: &str) -> HashMap<String, Instruction> {
    let data = read_to_string(filename).unwrap();
    let mut wires: HashMap<String, Instruction> = HashMap::new();
    for line in data.split('\n') {
        let mut sections = line.split(" -> ");
        let left = sections.next().unwrap();
        let right = sections.next().unwrap().to_owned();
        let expr= left.split_ascii_whitespace().collect::<Vec<_>>();
        match expr.len() {
            1 => {
                let val = if let Ok(n) = expr[0].parse::<u16>() {
                    Value::Literal(n)
                } else {
                    Value::Register(expr[0].to_owned())
                };
                wires.insert(right,Instruction::NoOp(val));
            }
            2 => {
                let val = if let Ok(n) = expr[1].parse::<u16>() {
                    Value::Literal(n)
                } else {
                    Value::Register(expr[1].to_owned())
                };
                wires.insert(right,Instruction::Not(val));

            },
            3 => {
                let lval = if let Ok(n) = expr[0].parse::<u16>() {
                    Value::Literal(n)
                } else {
                    Value::Register(expr[0].to_owned())
                };
                let rval = if let Ok(n) = expr[2].parse::<u16>() {
                    Value::Literal(n)
                } else {
                    Value::Register(expr[2].to_owned())
                };
                match expr[1] {
                    "AND" => {
                        wires.insert(right,Instruction::And(lval,rval));

                    },
                    "OR" => {
                        wires.insert(right,Instruction::Or(lval,rval));

                    },
                    "LSHIFT" => {
                        wires.insert(right,Instruction::Lshift(lval,rval));

                    },
                    "RSHIFT" => {
                        wires.insert(right,Instruction::Rshift(lval,rval));
                    },
                    _ => unimplemented!()
                }
            }
            _ => unimplemented!()
        };
        // wires.insert(right, result);
    }
    
    wires
}
