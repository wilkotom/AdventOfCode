use std::{fs::read_to_string,thread, time::Duration, sync::mpsc::{Receiver, Sender}};
use itertools::Itertools;

use intcode::intcode::start_machine;

/*

A partially automated solution. Play the game manually to explore the spaceship, then when 
you've collected all the various objects, type "solve" and the interpreter will brute force its
way through all different combinations of items until one lets you through.

"replay" will run a set of predefined steps as defined below.

*/

fn main() {
    let program = read_to_string("./day25/input.txt").unwrap().split(',').map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
    play(&program)
}

fn play(program: &[i64]) {

    let replay = "south
south
take spool of cat6
west
take space heater
north
take weather machine
north
west
west
take whirled peas
east
east
south
west
south
south
take space law space brochure
west
east
north
east
take candy cane
west
north
east
south
south
take shell
north
east
north
south
west
east
east
south
take hypercube
north
south
south
south
east\n";
    let (input,output) = start_machine(program);
    let mut recording = String::new();
    let mut last_description = String::new();
    'outer: loop {
        thread::sleep(Duration::from_micros(10000));
        let mut description = String::new();
        let in_stream = output.try_iter();
        for n in in_stream {
        
            description.push(n as u8 as char);
            thread::sleep(Duration::from_micros(100));
        }
        println!("{}", description);
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        recording.push_str(&line);
        if line == "solve\n" {
            
            println!("Solving");
            let line = "inv\n";
            for c in line.chars() {
                if input.send(c as u8 as i64).is_err() {
                    break 'outer
                }
            }
            thread::sleep(Duration::from_micros(10000));
            let mut description = String::new();
            let in_stream = output.try_iter();
            for n in in_stream {
                description.push(n as u8 as char);
                thread::sleep(Duration::from_micros(100));
            }
            let mut inventory = vec![];
            let mut items = description.split("\n\n").next().unwrap().split("\n- ");
            items.next();
            for item in items {
                inventory.push(item);
            }

            for item in inventory.iter() {
                let line = format!("drop {}\n", item);
                // print!("{}", line);
                for c in line.chars() {
                    if input.send(c as u8 as i64).is_err() {
                        break 'outer
                    }
                }

            let items = inventory.len();
            for i in 1..= items {
                for combo in inventory.clone().iter().combinations(i) {
                    for item in combo.iter() {
                        let line = format!("take {}\n", item);
                        for c in line.chars() {
                            if input.send(c as u8 as i64).is_err() {
                                break 'outer
                            }
                        }
                        thread::sleep(Duration::from_micros(10000));
                        let mut description = String::new();
                        let in_stream = output.try_iter();
                        for n in in_stream {
                        
                            description.push(n as u8 as char);
                            thread::sleep(Duration::from_micros(100));
                        }
                        // println!("{}", description);
                    }
                    let line = "east\n";
                    for c in line.chars() {
                        if input.send(c as u8 as i64).is_err() {
                            break 'outer
                        }
                    }
                    thread::sleep(Duration::from_micros(10000));
                    let mut description = String::new();
                    let in_stream = output.try_iter();
                    for n in in_stream {
                    
                        description.push(n as u8 as char);
                        thread::sleep(Duration::from_micros(100));
                    }
                    last_description = description.clone();
                    // println!("{}", description);
                    for item in combo.iter() {
                        let line = format!("drop {}\n", item);
                        for c in line.chars() {
                            if input.send(c as u8 as i64).is_err() {
                                break 'outer
                            }
                        }
                        // print!("{}", line);

                    }
                }
                thread::sleep(Duration::from_micros(10000));
                let mut description = String::new();
                let in_stream = output.try_iter();
                for n in in_stream {
                
                    description.push(n as u8 as char);
                    thread::sleep(Duration::from_micros(100));
                }
                // last_description = description;
                // println!("{}", description);
        

            }
        }
            
        } else if line == "replay\n" {
            println!("Replaying");
            for c in replay.chars() {
                if input.send(c as u8 as i64).is_err() {
                    break 'outer
                }
            }

        }else {
            for c in line.chars() {
                if input.send(c as u8 as i64).is_err() {
                    break 'outer
                }
            }
        }

    }
    println!("{}", last_description);
    println!("{}", recording);

}