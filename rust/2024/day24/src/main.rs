use std::{collections::{HashMap, HashSet, VecDeque}, error::Error, i64};

use aochelpers::get_daily_input;

#[derive(Debug,Clone, Copy, PartialEq, Eq)]
enum Operator {
    And,
    Or,
    Xor
}

#[derive(Debug, Clone)]
struct Operation {
    left: String,
    right: String,
    operator: Operator,
    destination: String 
}
fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(24,2024)?;
    let (registers, instructions) = parse_data(&data);
    println!("{}", part1(registers.clone(), instructions.clone()));
    println!("{}", part2(instructions));
    Ok(())
}

fn part2(instructions: VecDeque<Operation>) -> String {
    let mut bad_destinations = HashSet::new();
    for instr in instructions.iter() {
        if instr.destination.starts_with('z') && instr.operator != Operator::Xor && instr.destination != *"z45"{
            bad_destinations.insert(&instr.destination);
        } 
        if !(instr.destination.starts_with('z') || instr.left.starts_with('x') && instr.right.starts_with('y') || instr.left.starts_with('y') && instr.right.starts_with('x')) && instr.operator == Operator::Xor {
            bad_destinations.insert(&instr.destination);
        }
        if instr.operator == Operator::Xor && ((instr.left.starts_with('x') && instr.right.starts_with('y') )|| (instr.left.starts_with('y') && instr.right.starts_with('x')) ) && instr.destination != "z00" && !instructions.iter().any(|next_stage| next_stage.operator == Operator::Xor && (next_stage.left == instr.destination || next_stage.right == instr.destination)) {
            bad_destinations.insert(&instr.destination);
        }
        if instr.operator == Operator::And && instr.left != "y00" && !instructions.iter().any(|next_stage| next_stage.operator == Operator::Or && (next_stage.left == instr.destination || next_stage.right == instr.destination)) {
            bad_destinations.insert(&instr.destination);
        }
    }
    let mut dests = bad_destinations.iter().collect::<Vec<_>>();
    dests.sort();
    dests[..].iter().map(|c| c.to_string()).collect::<Vec<_>>().join(",")
}


fn part1(mut registers: HashMap<String, bool>, mut instructions: VecDeque<Operation>) -> i64 {
    while let Some (instr) = instructions.pop_front() {
        if let (Some(left) , Some(right)) = (registers.get(&instr.left), registers.get(&instr.right)) {
            registers.insert(instr.destination, match instr.operator {
                Operator::And => left & right,
                Operator::Or => left | right,
                Operator::Xor => left ^ right
            });
        } else {
            instructions.push_back(instr);
        }
    }
    let mut zeds = registers.keys().filter(|k| k.starts_with("z")).collect::<Vec<_>>();
    zeds.sort();
    let mut result = 0;
    for zed in zeds.iter().rev() {
        result *= 2;
        result += *registers.get(*zed).unwrap() as i64;
    }
    result
}

fn parse_data(data: &str) -> (HashMap<String, bool>, VecDeque<Operation>) {
    let mut registers = HashMap::new();
    let mut instructions = VecDeque::new();
    let mut sections = data.split("\n\n");
    for line in sections.next().unwrap().lines() {
        registers.insert(line[0..3].to_string(), line[5..].parse::<u8>().expect("Couldn't parse bit value") !=0);
    }

    for line in sections.next().unwrap().lines() {
        let mut tokens = line.split_ascii_whitespace();
        let left = tokens.next().expect("No left-hand argument").to_string();
        let operator = match tokens.next() {
            Some("AND") => Operator::And,
            Some("OR") => Operator::Or,
            Some("XOR") => Operator::Xor,
            _ => unimplemented!()
        };
        let right = tokens.next().expect("No right-hand argument").to_string();
        tokens.next();
        let destination = tokens.next().expect("No destination supplied").to_string();
        instructions.push_back(Operation { left, right, operator, destination});
    }


    (registers,instructions)
}

#[cfg(test)]
mod tests {

    use super::*;

    const TESTDATA: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

    #[test]
    fn test_part1() {
        let (registers, instructions) = parse_data(TESTDATA);
        assert_eq!(part1(registers, instructions), 2024);
    }

    #[test]
    fn test_part2() {
    }
}