use std::fs;
use std::collections::HashMap;


#[derive(Copy, Clone,Debug)]
struct MaskSet {
    zeroes: i64,
    ones: i64,
    exes: i64
}

#[derive(Copy, Clone,Debug)]
struct MemoryChange {
    address: i64,
    value: i64
}

#[derive(Debug)]
struct Instruction {
    mask_set: MaskSet,
    memory_changes: Vec<MemoryChange>
}


fn main() {
    part1(&read_file("./input.txt".to_string()));
    part2(&read_file("./input.txt".to_string()));

}

fn part1(program: &Vec<Instruction>) {
    let mut memory: HashMap<i64,i64> = HashMap::new();

    for instruction in program {
        for update in instruction.memory_changes.iter() {
            memory.insert(update.address, (update.value| instruction.mask_set.ones) & !instruction.mask_set.zeroes);

        }
    }
    let total:i64 = memory.values().sum();
    println!("Part 1 Answer: {}", total);
}

fn part2(program: &Vec<Instruction>) {
    let mut memory: HashMap<i64,i64> = HashMap::new();
    for instruction in program {
        let crystallised_bits = crystallise_bits(instruction.mask_set.exes);
        for update in instruction.memory_changes.iter() {
            let mut base_addresses: Vec<i64> = Vec::new();
            let base_address = update.address & !instruction.mask_set.exes | instruction.mask_set.ones & !instruction.mask_set.zeroes;
            for crystal in crystallised_bits.iter() {
                base_addresses.push(base_address + crystal);
            }
            for address in base_addresses.iter() {
                memory.insert(*address, update.value);
            }
        }
    }
    let total:i64 = memory.values().sum();
    println!("Part 2 Answer: {}", total);
}

fn crystallise_bits(exes: i64) -> Vec<i64> {
    let mut crystallised_bits: Vec<i64> = Vec::new();
    for power in 0..36 {
        if (exes | i64::pow(2,power)) == exes {
            crystallised_bits.push(i64::pow(2,power));
        }
    }
    let crystal_values = collapse_crystals(&crystallised_bits[0..]);
    crystal_values
}

fn collapse_crystals(values: &[i64]) -> Vec<i64> {
    let mut crystals: Vec<i64> = Vec::new();
    if values.len() == 1 {
        crystals.push(0);
        crystals.push(values[0]);
    } else if values.len() > 1 {
        let next_values = collapse_crystals(&values[1..]);
        for value in next_values {
            crystals.push(value);
            crystals.push(values[0] + value);
        }
    }
    crystals
}


fn read_file(filename: String) -> Vec<Instruction> {
    let raw_file = fs::read_to_string(&filename);
    let mut instructions: Vec<Instruction> = Vec::new();
    match raw_file {
        Ok(lines) => {
            let mut mask_set = MaskSet{zeroes:0, ones:0, exes:0}; 
            let lines: Vec<_> = lines.split("\n").collect();
            let mut memory_changes: Vec<MemoryChange> = Vec::new();
            for line in lines {
                let tokens: Vec<_> = line.split(" = ").collect();
                match tokens[0] {
                    "mask" => {
                        let instruction = Instruction{mask_set, memory_changes: memory_changes.clone()};
                        instructions.push(instruction);
                        memory_changes = Vec::new();
                        let bit_mask=tokens[1].to_string();
                        mask_set = parse_bit_mask(bit_mask);

                    }
                    _ => {
                        let value: i64 = tokens[1].parse().unwrap();
                        let address: i64 = tokens[0][4..tokens[0].len()-1].parse().unwrap();
                        memory_changes.push(MemoryChange{address,value});
                    }
                }
            }
            let instruction = Instruction{mask_set, memory_changes: memory_changes.clone()};
            instructions.push(instruction);
        }
        _ => {
            println!("{} not found", &filename);
            std::process::exit(1);
        }
    }

    instructions
}

fn parse_bit_mask(bit_mask: String) -> MaskSet{
    let mut zeroes: i64 = 0;
    let mut ones: i64 = 0;
    let mut exes: i64 = 0;
    for n in (0..36).rev() {
        match bit_mask.chars().nth(n) {
            Some('0') => {
                zeroes += i64::pow(2, (35-n) as u32);
            },
            Some('1') => {
                ones += i64::pow(2, 35-n as u32);
            },
            Some('X') => {
                exes += i64::pow(2, 35 -n as u32)
            },
            _ => {}
        }
    }
    MaskSet{zeroes, ones, exes}
}