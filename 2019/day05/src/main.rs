use std::fs::read_to_string;
use intcode::intcode::start_machine;

fn main() {
    let program = read_to_string("./day05/input.txt").unwrap().split(',').map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
    let (input, output) = start_machine(&program);
    input.send(1).unwrap();
    while let Ok(result) = output.recv() {
        if result != 0 {
            println!("Part 1: {}", result);
        }
    }

    let (input, output) = start_machine(&program);
    input.send(5).unwrap();
    while let Ok(result) = output.recv() {
        if result != 0 {
            println!("Part 1: {}", result);
        }
    }

}
