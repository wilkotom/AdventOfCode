use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct ProgramLine{
    instruction: String,
    arguments: [i32; 3]
}

fn main() {
    let prog_details = read_program("./input.txt");
    run_program(prog_details.0,prog_details.1);
}

fn run_program(ip: usize, program: Vec<ProgramLine>) {
    let mut registers = [0;6];
    registers[0] = 1;
    while registers[ip] < program.len() {
        if registers[ip] == 2 {
            println!("Need sum of factors for {}", registers[3])
        } 
        let line = &program[registers[ip]];
        // println!("{:?} {:?}", registers, line);
        execute(&line.instruction, &line.arguments, &mut registers);
        registers[ip] += 1;
        
    }
    println!("{:?}", registers)
}

fn read_program(filename: &str) -> (usize, Vec<ProgramLine>) {
    let data = read_to_string(filename).unwrap();
    let mut program = Vec::new();
    // let mut registers = [0_usize; 6];
    let mut ip = 0;
    for line in data.split('\n') {
        let mut words = line.split_ascii_whitespace();
        if line.starts_with('#') {
            words.next();
            ip=words.next().unwrap().parse::<usize>().unwrap();
        } else {
            let mut words = line.split_ascii_whitespace();
            program.push(ProgramLine{ instruction: words.next().unwrap().to_string(),
                                      arguments: [words.next().unwrap().parse::<i32>().unwrap(), words.next().unwrap().parse::<i32>().unwrap(), words.next().unwrap().parse::<i32>().unwrap()]});
        }
    }
    (ip, program)
}

fn execute(instr: &str, args: &[i32], registers: &mut [usize; 6]) {

    match instr {
        "addr" => {
            registers[args[2] as usize] = registers[args[0] as usize] + registers[args[1] as usize];
        },
        "addi" => {
            registers[args[2] as usize] = registers[args[0] as usize] + args[1] as usize;
        },
        "mulr" => {
            registers[args[2] as usize] = registers[args[0] as usize] * registers[args[1] as usize];
        },
        "muli" => {
            registers[args[2] as usize] = registers[args[0] as usize] * args[1] as usize;
        },
        "banr" => {
            registers[args[2] as usize] = registers[args[0] as usize] & registers[args[1] as usize];
        },
        "bani" => {
            registers[args[2] as usize] = registers[args[0] as usize] & args[1] as usize;
        },
        "borr" => {
            registers[args[2] as usize] = registers[args[0] as usize] | registers[args[1] as usize];
        },
        "bori" => {
            registers[args[2] as usize] = registers[args[0] as usize] | args[1] as usize;
        },
        "setr" => {
            registers[args[2] as usize] = registers[args[0] as usize]; 
        },
        "seti" => {
            registers[args[2] as usize] = args[0] as usize;
        },
        "gtir" => {
            registers[args[2] as usize] = if args[0] as usize > registers[args[1] as usize] {1} else {0};
        },
        "gtri" => {
            registers[args[2] as usize] = if registers[args[0] as usize] > args[1] as usize {1} else {0};
        },
        "gtrr" => {
            registers[args[2] as usize] = if registers[args[0] as usize] > registers[args[1] as usize] {1} else {0};
        },
        "eqir" => {
            registers[args[2] as usize] = if args[0] as usize == registers[args[1] as usize] {1} else {0};
        },
        "eqri" => {
            registers[args[2] as usize] = if registers[args[0] as usize] == args[1] as usize {1} else {0};
        },
        "eqrr" => {
            registers[args[2] as usize] = if registers[args[0] as usize] == registers[args[1] as usize] {1} else {0};
        },
        _ => unimplemented!()
    }
}