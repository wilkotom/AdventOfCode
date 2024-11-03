use std::fs;

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
    Forward,
    Left,
    Right,
    Unknown
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    value: i32
}

#[derive(Debug)]
struct Ferry {
    x: i32,
    y: i32,
    bearing: i32,
}

fn main() {
    let instructions = read_instructions("./input.txt".to_string());
    part1(&instructions);
    part2(&instructions);

}

fn read_instructions(filename: String) -> Vec<Instruction> {
    let mut directions: Vec<Instruction> = Vec::new();
    let raw_file = fs::read_to_string(&filename);
    match raw_file {
        Ok(lines) => {
            let lines: Vec<_> = lines.split("\n").collect();
            for line in lines {
                let direction = match line.chars().next() {
                    Some('N') => Direction::North,
                    Some('S') => Direction::South,
                    Some('E') => Direction::East,
                    Some('W') => Direction::West,
                    Some('L') => Direction::Left,
                    Some('R') => Direction::Right,
                    Some('F') => Direction::Forward,
                    _ => Direction::Unknown
                };
                let value: i32 = line[1..].parse().unwrap();
                directions.push(Instruction{ direction, value })
            }
        }
        _ => {
            println!("{} not found", &filename);
            std::process::exit(1);
        }
    }
    directions
}

fn part1(instructions: &[Instruction]) {
    let mut ferry = Ferry{x:0, y:0, bearing: 90};
    for instruction in instructions.iter() {
        match instruction.direction {
            Direction::North => {
                ferry.y += instruction.value;
            },
            Direction::South => {
                ferry.y -= instruction.value;
            },
            Direction::East => {
                ferry.x += instruction.value;
            },
            Direction::West => {
                ferry.x -= instruction.value;
            },
            Direction::Left => {
                ferry.bearing = (ferry.bearing - instruction.value + 360) % 360;
            },
            Direction::Right => {
                ferry.bearing = (ferry.bearing + instruction.value) % 360;
            },
            Direction::Forward => {
                match ferry.bearing {
                    0 => {ferry.y += instruction.value;},
                    90 => {ferry.x += instruction.value;},
                    180 => {ferry.y -= instruction.value;},
                    270 => {ferry.x -= instruction.value;},
                    _ => {}
                }
            },
            _ => {}
        }   
    }
    println!{"Part 1 answer: {}", ferry.x.abs() + ferry.y.abs()}    
}


fn part2(instructions: &[Instruction]) {
    let mut ferry = Ferry{x:0, y:0, bearing: 90};
    let mut waypoint = Ferry{x:10, y:1, bearing: 0};
    for instruction in instructions.iter() {
        match instruction.direction {
            Direction::North => {
                waypoint.y += instruction.value;
            },
            Direction::South => {
                waypoint.y -= instruction.value;
            },
            Direction::East => {
                waypoint.x += instruction.value;
            },
            Direction::West => {
                waypoint.x -= instruction.value;
            },
            Direction::Right => {
                let mut degrees = instruction.value;
                while degrees > 0 {
                    let current_x = waypoint.x;
                    let current_y = waypoint.y;    
                    waypoint.x = current_y;
                    waypoint.y = -current_x;
                    degrees -= 90;
                }
            },
            Direction::Left => {
                let mut degrees = instruction.value;
                while degrees > 0 {
                    let current_x = waypoint.x;
                    let current_y = waypoint.y;    
                    waypoint.x = -current_y;
                    waypoint.y = current_x;
                    degrees -= 90;
                }
            },
            Direction::Forward => {
                ferry.x += waypoint.x * instruction.value;
                ferry.y += waypoint.y * instruction.value;
            },
            _ => {}
        }
    }
    println!{"Part 1 answer: {}", ferry.x.abs() + ferry.y.abs()}    
}