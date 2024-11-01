use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug, Copy, Clone)]
enum Holding {
    Chip(i32),
    Nothing
}

#[derive(Debug, Copy, Clone)]
enum Destination {
    Bot(i32),
    Output(i32)
}

#[derive(Debug, Copy, Clone)]
struct Robot {
    id: i32,
    holding: Holding,
    high: Destination,
    low: Destination
}

#[derive(Debug)]
struct Assignment {
    value: i32,
    destination: Destination
}

fn main() {
    let file = std::fs::read_to_string(String::from("./input.txt")).unwrap_or(String::from(""));
    let mut assignments: VecDeque<Assignment> = VecDeque::new();
    let mut robots: HashMap<i32, Robot> = HashMap::new();
    let mut outputs: HashMap<i32,i32> = HashMap::new();

    for line in file.split("\n"){
        let tokens = line.split_ascii_whitespace().collect::<Vec<&str>>();
        match tokens[0] {
         "bot" => {
             let bot_id = tokens[1].parse::<i32>().unwrap();
             let low = match tokens[5] {
                "output" => { Destination::Output(tokens[6].parse::<i32>().unwrap())},
                _ => { Destination::Bot(tokens[6].parse::<i32>().unwrap())}};
            let high = match tokens[10] {
                "output" => { Destination::Output(tokens[11].parse::<i32>().unwrap())},
                _ => { Destination::Bot(tokens[11].parse::<i32>().unwrap())}};
            
            robots.insert(bot_id, Robot{id: bot_id, holding: Holding::Nothing, high, low});

         },
         "value" => {
            let assignment = Assignment{value: tokens[1].parse::<i32>().unwrap(), 
                                                  destination: Destination::Bot(tokens[5].parse::<i32>().unwrap())};
            assignments.push_back(assignment);
            
         },
         _ => {}
        }
    }

    while assignments.len() > 0 {
        let assignment = assignments.pop_front().unwrap();
        match assignment.destination {
            Destination::Bot(n) => {
                let mut robot = robots.get_mut(&n).unwrap();
                match robot.holding {
                    Holding::Nothing => {
                        robot.holding = Holding::Chip(assignment.value);
                    }
                    Holding::Chip(x) => {
                        if assignment.value == 61 && x == 17 ||
                            assignment.value == 17 && x == 61 {
                                println!("Robot {} compares 17 and 61", robot.id);
                            }
                        if assignment.value > x {
                            assignments.push_back(Assignment{value: assignment.value, destination: robot.high});
                            assignments.push_back(Assignment{value: x, destination: robot.low});
                        } else {
                            assignments.push_back(Assignment{value: assignment.value, destination: robot.low});
                            assignments.push_back(Assignment{value: x, destination: robot.high});

                        }
                        robot.holding = Holding::Nothing;
                    }
                }
            }
            Destination::Output(x) => {
                outputs.insert(x, assignment.value);
            }
        }

    }
    println!("Part 2 answer: {:?}", outputs.get(&0).unwrap() * outputs.get(&1).unwrap() * outputs.get(&2).unwrap());
}
