use std::collections::VecDeque;

static PASSCODE: &str = "rrrbmfta";

#[derive(Debug)]
struct Coordinate {
    x: isize,
    y: isize
}

fn main() {
    let mut next_steps: VecDeque<String> = VecDeque::new();
    next_steps.push_back(String::from(PASSCODE));
    while next_steps.len() > 0 {
        let mut location = Coordinate{x: 0, y: 0};
        let directions = next_steps.pop_front().unwrap();
        for direction in directions[PASSCODE.len()..].chars() {
            match direction {
                'U' => location.y -= 1,
                'D' => location.y += 1,
                'L' => location.x -= 1,
                'R' => location.x += 1,
                _ => {}
            }
        }
        if location.x ==3 && location.y ==3 {
            println!("{:?} {} {}", location, &directions[PASSCODE.len()..], &directions[PASSCODE.len()..].len());
            // break;
        } else {
    
        
        let md5sum = format!("{:x}", md5::compute(&directions));
        match md5sum.chars().nth(0).unwrap() {
            'b' | 'c' | 'd' | 'e' | 'f' => {
                if location.y > 0 {
                    let next = format!("{}U", directions);
                    next_steps.push_back(next);
                }
            },
            _ => {}
        }
        match md5sum.chars().nth(1).unwrap() {
            'b' | 'c' | 'd' | 'e' | 'f' => {
                if location.y < 3 {
                    let next = format!("{}D", directions);
                    next_steps.push_back(next);
                }
            },
            _ => {}
        }
        match md5sum.chars().nth(2).unwrap() {
            'b' | 'c' | 'd' | 'e' | 'f' => {
                if location.x > 0 {
                    let next = format!("{}L", directions);
                    next_steps.push_back(next);
                }
            },
            _ => {}
        }
        match md5sum.chars().nth(3).unwrap() {
            'b' | 'c' | 'd' | 'e' | 'f' => {
                if location.x < 3 {
                    let next = format!("{}R", directions);
                    next_steps.push_back(next);
                }
            },
            _ => {}
        }
        }
    }
}
