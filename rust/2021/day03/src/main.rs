fn main() {
    let data = std::fs::read_to_string("./input.txt").unwrap();
    let lines:Vec<&str> = data.split("\n").collect::<Vec<_>>();

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines, true) * part2(&lines, false));
}

fn part1(lines: &[&str]) -> i32 {
    let mut gamma = 0;
    let mut epsilon = 0;
    for pos in 0..lines[0].len(){
        gamma <<=1;
        epsilon <<=1;
        let ones_count = lines.iter().map(|x| x.chars().nth(pos).unwrap()).filter(|x| *x == '1').count();
        if ones_count >= (lines.len() - ones_count) {
            gamma += 1;
        } else {
            epsilon +=1;
        }
    }
    gamma * epsilon
}

fn part2(lines: &[&str], most: bool) -> i32 {
    let mut pos = 0;
    let mut values = lines.to_owned();
    while values.len() > 1 {
        let ones_count = values.iter().map(|x| x.chars().nth(pos).unwrap()).filter(|x| *x == '1').count();
        if ones_count >= (values.len() - ones_count) {
            values.retain(|x|x.chars().nth(pos).unwrap() == if most {'1'} else {'0'})
        } else {
            values.retain(|x|x.chars().nth(pos).unwrap() == if most {'0'} else {'1'})
        }
        pos += 1
    }
    i32::from_str_radix(values[0],2).unwrap()
}