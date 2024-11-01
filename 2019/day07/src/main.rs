
use std::fs::read_to_string;
use intcode::intcode::start_machine;
use itertools::Itertools;

fn main() {
    let program = read_to_string("./day07/input.txt").unwrap().split(',').map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
    let phases: Vec<i64> = vec![0,1,2,3,4];
    let mut result = 0;
    for phase_setting in phases.iter().permutations(5) {
        result = result.max(test_signal(&program, &phase_setting));
    
    }
    println!("Part 1: {:?}", result);
    let phases: Vec<i64> = vec![5,6,7,8,9];
    result = 0;
    for phase_setting in phases.iter().permutations(5) {
        result = result.max(test_signal(&program, &phase_setting));
    
    }
    println!("Part 2: {:?}", result);

}

fn test_signal(program: &[i64], phase_setting: &[&i64]) -> i64 {

    let mut inputs = Vec::new();
    let mut outputs = Vec::new();
    for p in phase_setting.iter().take(5) {
        let (input, output) = start_machine(program);
        input.send(**p).unwrap();
        inputs.push(input);
        outputs.push(output);
    }

    let mut dest = 0;
    let mut signal = 0;
    while inputs[dest].send(signal).is_ok() {
        
        signal = outputs[dest].recv().unwrap();
        dest = (dest +1) % 5;
    }

    signal
}