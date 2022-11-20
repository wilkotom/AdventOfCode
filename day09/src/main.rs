use std::fs::read_to_string;
use intcode::intcode::start_machine;

fn main() {
    let program = read_to_string("./day09/input.txt").unwrap().split(',').map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
    let (input, output) = start_machine(&program);
    input.send(1).unwrap();
    while let Ok(n) = output.recv() {
        println!("Part 1: {}", n);
    }
    let (input, output) = start_machine(&program);
    input.send(2).unwrap();
    while let Ok(n) = output.recv() {
        println!("Part 2: {}", n);
    }
}
