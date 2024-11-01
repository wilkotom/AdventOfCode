use std::fs::read_to_string;

fn main() {
    let data = read_to_string("./input.txt").unwrap();
    part1(&data);
    part2(&data);
}

fn part1(data: &str) {
    let mut total_chars = 0;
    let mut part1_len = 0;
    for line in data.split('\n') {
        part1_len += line.len();
        let mut chars = line[1..line.len()-1].chars();
        while let Some(c) = chars.next(){
            match c{
                '\\' => {
                    total_chars +=1;
                    let c1 = chars.next();
                    match c1 {
                        Some('x') => {
                            chars.next();
                            chars.next();
                        }
                        Some('\\') | Some('"') => {}
                        Some(_) => {
                            unimplemented!();}
                        _ => {}
                    }
                },
                _ => {
                    total_chars +=1;
                }
                
            }
        }
    }
    println!("Part 1: {}", part1_len - total_chars);
}

fn part2(data: &str) {
    let mut total_chars = 0;
    let mut part1_len = 0;
    for line in data.split('\n') {
        part1_len += line.len();
        for c in line.chars(){
            match c{
                '"' | '\\' => {total_chars +=2;}
                _ => {total_chars +=1;}

            }
        }
        total_chars +=2
    }
    println!("Part 2: {}", total_chars - part1_len);
}