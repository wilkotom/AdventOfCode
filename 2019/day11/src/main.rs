use std::{fs::read_to_string, collections::HashMap, sync::mpsc::RecvError};
use intcode::intcode::start_machine;

fn main() {
    let program = read_to_string("./day11/input.txt").unwrap().split(',').map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
    start_robot(&program, false);
    start_robot(&program, true);

}

fn start_robot (program: &[i64], part2: bool){
    let (input, output) = start_machine(program);
    let mut facing = (0,1);
    let mut location = (0,0);
    let mut panels = HashMap::new();
    panels.insert((0,0), part2);
    while input.send(if *panels.get(&location).unwrap_or(&false) {1} else {0}).is_ok() {
        
        let colour = output.recv();
        match colour {
            Ok(0) => {panels.insert(location, false)},
            Ok(1) => {panels.insert(location, true)},
            Err(RecvError) => break,
            _ => unimplemented!()
        };
        let direction = output.recv();
        facing = match direction {
            Ok(0) => (-facing.1, facing.0),
            Ok(1) => (facing.1, -facing.0),
            Err(RecvError) => unimplemented!(),
            _ => unimplemented!()
        };
        location.0 += facing.0;
        location.1 += facing.1;
    }
    if !part2 {
        println!("Part 1: {:?}", panels.len());
    } else {
        println!("Part 2:");
        let min_x = panels.keys().map(|c| c.0).min().unwrap();
        let max_x = panels.keys().map(|c| c.0).max().unwrap();
        let min_y = panels.keys().map(|c| c.1).min().unwrap();
        let max_y = panels.keys().map(|c| c.1).max().unwrap();

        for y in (min_y..max_y +1).rev() {
            for x in min_x..max_x +1 {
                print!("{}", if *panels.get(&(x,y)).unwrap_or(&false) {"#"} else {" "});
            }
            println!();
        }
    }
}