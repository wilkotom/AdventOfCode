use std::{fs::read_to_string, collections::HashSet, thread, time::Duration};

use intcode::intcode::start_machine;

#[derive(Debug,Clone,Copy, Hash, PartialEq, Eq)]
struct Coordinate {
    x: i64,
    y: i64
}

impl Coordinate {
    fn neighbours(&self) -> [Coordinate; 4] {
        [ Coordinate{x: self.x -1, y: self.y},
        Coordinate{x: self.x +1, y: self.y},
        Coordinate{x: self.x, y: self.y -1},
        Coordinate{x: self.x, y: self.y +1},
        ]
    }
}

fn main() {
    let program = read_to_string("./day17/input.txt").unwrap().split(',').map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
    let track_directions = part1(&program);
    let directions = split_directions(track_directions);
    part2(program, directions);
}

fn part2(mut program:  Vec<i64>, directions: [String;4]) {
    program[0] = 2;
    println!("{:?}", directions);
    let (input, output) = start_machine(&program);
    for instruction in &directions {
        let in_stream = output.try_iter();
        thread::sleep(Duration::from_millis(5));
        for n in in_stream {
            print!("{}", n as u8 as char);
        }
        let response = format!("{}\n",instruction);
        for c in response.chars() {
            print!("{}",c);
            input.send(c as u8 as i64).unwrap();
        }
    }
    let in_stream = output.try_iter();
    thread::sleep(Duration::from_millis(100));
    for n in in_stream {
        print!("{}", n as u8 as char);
    }
    for c in "n\n".chars() {
        print!("{}",c);
        input.send(c as u8 as i64).unwrap();

    }
    let in_stream = output.try_iter();
    thread::sleep(Duration::from_millis(100));
    for n in in_stream {
        if n > 255{
            println!("Dust collected: {}", n);
        }
        else {        
            print!("{}", n as u8 as char);
        }
    }
}

fn split_directions(mut directions: String) -> [String;4] {
    let mut section_len = 4;
    let mut section_a = directions[..section_len].to_owned();
    while directions.matches(&section_a).count() > 1 && section_a.len() <= 20 {
        section_len +=4;
        while directions.chars().nth(section_len -1) != Some(',') {
            section_len +=1;
        }
        section_a = directions[..section_len].to_owned();

    }
    section_a = directions[..section_len-4].to_owned();
    directions = directions.replace(&section_a, "A,");
    section_a = section_a.strip_suffix(',').unwrap().to_owned();

    let mut section_b = directions[directions.len()-20.. if directions.ends_with('A') {directions.len() -2} else {directions.len()}].to_owned();
    while section_b.contains('A') || section_b.len() > 20 || section_b.starts_with(',') {
        section_b = section_b[1..].to_owned();
    }
    directions = directions.replace(&section_b, "B");

    let mut section_c_start = 0;
    while directions.chars().nth(section_c_start) != Some('R') && directions.chars().nth(section_c_start) != Some('L') {
        section_c_start +=1;
    }
    section_len = 1;
    while directions.matches(&directions[section_c_start..section_c_start+ section_len]).count() > 1 {
        section_len+=1;
    }
    let section_c = directions[section_c_start..section_len].to_owned();
    directions = directions.replace(&section_c, "C");    
    [directions, section_a, section_b, section_c]
}

fn part1(program: &[i64]) -> String{
    let (_, output) = start_machine(program);
    let mut x = 0;
    let mut y = 0;
    let mut grid = HashSet::new();
    let mut origin = Coordinate{x:-1, y:-1};
    let mut pointing = ' ';
    while let Ok(c) = output.recv() {
        match c as u8 as char {
            '#' => {
                grid.insert(Coordinate{x,y});
                x+=1;
            },
            '\n' => {
                x = 0;
                y += 1;
            },
            '^' | '>' | 'v' | '<'=> {
                grid.insert(Coordinate{x,y});
                origin = Coordinate{x,y};
                x+=1;
                pointing = c as u8 as char;

            }
            _ => {
                x+=1;
            }  
        }
    }
    let mut alignment = 0;
    for spot in &grid {
        if spot.neighbours().iter().all(|c| grid.contains(c)) {
            alignment += spot.x * spot.y;
        }
    }
    println!("Part 1 answer: Alignment: {}", alignment);

    let mut bot_location = origin;
    let mut directions = String::new();
    let mut path_counter = 0;

    while bot_location == origin || bot_location.neighbours().iter().filter(|c| grid.contains(c)).count() > 1 {
        match pointing {
            '^' => {
                if grid.contains(&Coordinate{x: bot_location.x, y: bot_location.y -1}) {
                    path_counter +=1;
                    bot_location.y -=1;
                } else if grid.contains(&Coordinate{x: bot_location.x -1, y: bot_location.y}) {
                        pointing = '<';
                        directions.push_str(&format!("{},L,", path_counter));
                        path_counter = 0;
                } else {
                    pointing = '>';
                    directions.push_str(&format!("{},R,", path_counter));
                    path_counter = 0;
                }
            },
            'v' => {
                if grid.contains(&Coordinate{x: bot_location.x, y: bot_location.y +1}) {
                    path_counter +=1;
                    bot_location.y +=1;
                } else if grid.contains(&Coordinate{x: bot_location.x -1, y: bot_location.y}) {
                        pointing = '<';
                        directions.push_str(&format!("{},R,", path_counter));
                        path_counter = 0;
                } else {
                    pointing = '>';
                        directions.push_str(&format!("{},L,", path_counter));
                        path_counter = 0;
                }
            },
            '>' => {
                if grid.contains(&Coordinate{x: bot_location.x+1 , y: bot_location.y}) {
                    path_counter +=1;
                    bot_location.x +=1;
                } else if grid.contains(&Coordinate{x: bot_location.x, y: bot_location.y -1}) {
                        pointing = '^';
                        directions.push_str(&format!("{},L,", path_counter));
                        path_counter = 0;
                } else {
                    pointing = 'v';
                        directions.push_str(&format!("{},R,", path_counter));
                        path_counter = 0;
                }
            },
            '<' => {
                if grid.contains(&Coordinate{x: bot_location.x -1, y: bot_location.y}) {
                    path_counter +=1;
                    bot_location.x -=1;
                } else if grid.contains(&Coordinate{x: bot_location.x, y: bot_location.y -1}) {
                        pointing = '^';
                        directions.push_str(&format!("{},R,", path_counter));
                        path_counter = 0;
                } else {
                    pointing = 'v';
                        directions.push_str(&format!("{},L,", path_counter));
                        path_counter = 0;
                }
            },
            _ => unimplemented!()
        }
        // println!("{}", directions);
        
    }
    directions.push_str(&format!("{}", path_counter));
    directions[2..].to_string()
}


#[test]
fn example_path() {
    let path = "R,8,R,8,R,4,R,4,R,8,L,6,L,2,R,4,R,4,R,8,R,8,R,8,L,6,L,2".to_owned();
    assert_eq!(["A,C,B,C,A,B", "R,8,R,8", "R,8,L,6,L,2", "R,4,R,4"], split_directions(path));
}