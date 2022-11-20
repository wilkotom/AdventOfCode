use std::collections::HashSet;

fn main() {
    let data = std::fs::read_to_string("./input.txt").unwrap();
    let mut visited: HashSet<(i32,i32)> = HashSet::new();
    let mut x = 0;
    let mut y = 0;
    visited.insert((x,y));
    for c in data.chars() {
        match c {
            '^' => {y += 1}
            '>' => {x += 1},
            '<' => {x -=1},
            'v' => {y -=1},
            _ => unimplemented!()
        }
        visited.insert((x,y));
    }
    println!("Part 1 {}", visited.len());

    let mut visited: HashSet<(i32,i32)> = HashSet::new();
    x = 0;
    y = 0;
    let mut rx = 0;
    let mut ry = 0;
    visited.insert((x,y));
    for (step, c) in data.chars().enumerate() {
        if step % 2 == 0 {
            match c {
                '^' => {y += 1}
                '>' => {x += 1},
                '<' => {x -=1},
                'v' => {y -=1},
                _ => unimplemented!()
            }
            visited.insert((x,y));
        } else {
            match c {
                '^' => {ry += 1}
                '>' => {rx += 1},
                '<' => {rx -=1},
                'v' => {ry -=1},
                _ => unimplemented!()
            }
            visited.insert((rx,ry));

        }
    }
    println!("Part 2: {}", visited.len());

}
