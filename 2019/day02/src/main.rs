use std::fs::read_to_string;
use intcode::intcode::run_program;

fn main() {
    let program = read_to_string("./day02/input.txt").unwrap().split(',').map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
    let mut new_prog = program.clone();
    new_prog[1] = 12;
    new_prog[2] = 2;
    println!("Part 1: {:?}", run_program(&program, 0));
    'noun_loop: for noun in 0..100{
        for verb in 0..100 {
            let mut new_prog = program.clone();
            new_prog[1] = noun; 
            new_prog[2] = verb;
            if run_program(&new_prog, 0) == 19690720 {
                println!("Part 2: {}", noun*100 + verb);
                break 'noun_loop;
            }
        }
    }
}
