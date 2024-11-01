use std::{fs::read_to_string, thread, time::Duration, collections::HashSet};
use intcode::intcode::start_machine;

fn main() {
    let program = read_to_string("./day23/input.txt").unwrap().split(',').map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();

    let mut inputs = Vec::new();
    let mut outputs = Vec::new();
    for i in 0..=49 {
        let (input, output) = start_machine(&program);
        input.send(i).unwrap();
        input.send(-1).unwrap();
        inputs.push(input);
        outputs.push(output);
    }

    let mut nat = (0,0);
    let mut seen = HashSet::new();
    let mut part1 = 0;

    loop {
        let mut transmitted = false;
        // If running in debug mode, this sleep needs to be longer...
        thread::sleep(Duration::from_micros(50));
        for output in outputs.iter().take(49 + 1) {
            let mut in_stream = output.try_iter();
            let receiver = in_stream.next();
            if let Some(n) = receiver {
                transmitted = true;
                let x = output.recv().unwrap();
                let y = output.recv().unwrap();
                if n != 255 {
                    inputs[n as usize].send(x).unwrap();
                    inputs[n as usize].send(y).unwrap();
                
                } else {
                    if part1 == 0 {
                        part1 = y;
                        println!("Part 1: {}", y);
                    }
                    nat = (x,y);
                }
            }

        }
        if ! transmitted {
            if seen.contains(&nat.1) {
                println!("Part 2: {}", nat.1);
                break;
            } else {
                inputs[0].send(nat.0).unwrap();
                inputs[0].send(nat.1).unwrap();
                seen.insert(nat.1);
            }
        }

    }
}
