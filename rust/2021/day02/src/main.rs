fn main() {
    let data = std::fs::read_to_string("./input.txt").unwrap();
    let instructions = data.split("\n").collect::<Vec<_>>();

    part1(&instructions);
    part2(&instructions);

}

fn part1(instructions: &[&str]) {
    let mut x: isize = 0;
    let mut y: isize = 0;

    for line in instructions {
        let instruction = line.split_ascii_whitespace().collect::<Vec<_>>();
        match instruction.get(0) {
            Some(&"forward") => {
               x += instruction.get(1).unwrap().parse::<isize>().unwrap();
            },
            Some(&"down") => {
                y +=  instruction.get(1).unwrap().parse::<isize>().unwrap();
            },
            Some(&"up") => {
                y -= instruction.get(1).unwrap().parse::<isize>().unwrap();
            },
            _ => {}   
        }
    }
    println!("Part 1: {}", x*y);
}

fn part2(instructions: &[&str]) {
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut aim:isize = 0;

    for line in instructions {
        let instruction = line.split_ascii_whitespace().collect::<Vec<_>>();
        match instruction.get(0) {
            Some(&"forward") => {
                let n = instruction.get(1).unwrap().parse::<isize>().unwrap();
                x += n;
                y += n* aim;
            },
            Some(&"down") => {
                aim += instruction.get(1).unwrap().parse::<isize>().unwrap();
            },
            Some(&"up") => {
                aim -= instruction.get(1).unwrap().parse::<isize>().unwrap();
            },
            _ => {}   
        }
    }
    println!("Part 2: {}", x*y);
}

