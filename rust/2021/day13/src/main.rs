use std::collections::HashSet;

#[derive(Debug,Copy,Clone,Hash,Eq,PartialEq)]
struct Coordinate{
    x: i64,
    y:i64
}
fn main() {
    let data = std::fs::read_to_string("./input.txt").unwrap();
    let mut sections= data.split("\n\n");
    let mut paper: HashSet<Coordinate> = HashSet::new();
    let mut max_x = 0;
    let mut max_y = 0;
    let mut part1: Option<usize> = None;
    for line in sections.next().unwrap().split('\n') {
        let mut nums = line.split(',');
        let x = nums.next().unwrap().parse::<i64>().unwrap();
        let y = nums.next().unwrap().parse::<i64>().unwrap();
        if x > max_x {
            max_x = x+1;
        }
        if y > max_y {
            max_y = y+1;
        }
        paper.insert(Coordinate{x,y});
    }
    for fold in sections.next().unwrap().split('\n') {
        let line = fold.split_ascii_whitespace().last().unwrap();
        let mut tokens = line.split('=');
        let axis = tokens.next().unwrap();
        let value = tokens.next().unwrap().parse::<i64>().unwrap();
        let current_dots = paper.iter().copied().collect::<Vec<_>>();
        match axis {
            "x" => {
                max_x = value;
                for dot in current_dots {
                    if dot.x > value {
                        let x = value -( dot.x- value);
                        let y = dot.y;
                        paper.insert(Coordinate{x,y});
                        paper.remove(&dot);
                    }
                }
            },
            "y" => {
                max_y = value;
                for dot in current_dots {
                    if dot.y > value {
                        let y = value -( dot.y- value);
                        let x = dot.x;
                        paper.insert(Coordinate{x,y});
                        paper.remove(&dot);
                    }
                }
            },
            _ => unreachable!()
        }
        if part1 == None {
            part1 = Some(paper.len());
        }
    }
    println!("Part 1 answer: {}", part1.unwrap());
    print_paper(&paper, max_x, max_y);

}

fn print_paper(paper: &HashSet<Coordinate>, max_x: i64, max_y: i64) {
    for y in 0..max_y {
        for x in 0..max_x {
            print!("{}", if paper.contains(&Coordinate{x,y}) {"██"} else {"  "});
        }
        println!();
    }
}