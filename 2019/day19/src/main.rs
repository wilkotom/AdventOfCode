use std::fs::read_to_string;

use intcode::intcode::start_machine;

fn main() {
    let program = read_to_string("./day19/input.txt").unwrap().split(',').map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
    println!("Part 1: {}", part1(&program));
    println!("Part 2: {}", part2(&program))

}


fn part1(program: &[i64]) -> i64 {
    let mut answer = 0;
    // let (input, output) = start_machine(program.clone());
    for x in 0..50 {
        for y in 0..50 {
            let (input, output) = start_machine(program);
            input.send(x).unwrap();
            input.send(y).unwrap();
            let result = output.recv().unwrap();
            answer += result;
        }
    }

    answer
}

fn part2( program: &[i64]) -> i64 {
    // target block is 100x100. Therefore the first possible place it could be is
    // at 0,99 if the bounds of the beam are the x and y axes
    let mut y =99;
    let mut x = 0;
    loop {
        while get_result(x, y, program) == 0 {
            x+=1;
        }
        if get_result(x +99, y -99, program) == 1 {
            return x * 10000 + y-99;
        }
        y +=1;
    }
}

fn get_result(x: i64, y: i64, program: &[i64]) -> i64 {
    let (input, output) = start_machine(program);
    input.send(x).unwrap();
    input.send(y).unwrap();
    output.recv().unwrap()
}