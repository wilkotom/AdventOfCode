use std::{fs::read_to_string, cmp::Ordering};
use intcode::intcode::start_machine;

fn main() {
    let mut program = read_to_string("./day13/input.txt").unwrap().split(',').map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
    program[0] = 2;
    let results = play_game(&program);
    println!("Part 1: {:?}", results.0);
    println!("Part 2: {:?}", results.1);
}


fn play_game(program: &[i64]) -> (usize, i64) {
    let (input, output) = start_machine(program);
    let mut paddle_x = 0;
    let mut block_count = 0;
    let mut score = 0;
    while let Ok (x) = output.recv(){
        let _ = output.recv();
        match output.recv() {
            Ok(n) if x == -1 => {score = n},
            Ok(0) | Ok(1) => {},
            Ok(2) => { block_count +=1; }
            Ok(3) => {paddle_x = x;},
            Ok(4) => { input.send(match paddle_x.cmp(&x) {
                Ordering::Less => 1,
                Ordering::Equal => 0,
                Ordering::Greater =>-1,
            }).unwrap();}
            Ok(_) => unimplemented!(),
            Err(_) => break
        };
    }
    (block_count, score)
}
