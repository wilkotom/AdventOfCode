use std::{fs::read_to_string, thread, time::Duration};

use intcode::intcode::start_machine;

fn main() {
    let program = read_to_string("./day21/input.txt").unwrap().split(',').map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
    program_springdroid(&program, "NOT A J\nNOT B T\nAND D T\nOR T J\nNOT C T\nAND D T\nOR T J\nWALK\n");
    program_springdroid(&program, "NOT A J\nNOT B T\nAND D T\nAND H T\nOR T J\nNOT C T\nAND D T\nAND H T\nOR T J\nRUN\n");
}

fn program_springdroid(program: &[i64], instructions: &str) {
    let (input, output) = start_machine(program);
    thread::sleep(Duration::from_millis(5));
    let in_stream = output.try_iter();
    for n in in_stream {
        print!("{}", n as u8 as char);
    }
    for c in instructions.chars() {
        print!("{}",c);
        input.send(c as u8 as i64).unwrap();
    }

    for n in output {
        if n <=255 {
        print!("{}", n as u8 as char);
        } else {
            print!("{}", n);
        }
    }
    println!()
}