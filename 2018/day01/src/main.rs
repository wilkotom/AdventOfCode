use std::collections::{HashSet};


fn main() {
    let data = std::fs::read_to_string("./input.txt")
        .unwrap()
        .split('\n')
        .map(|x| x.parse::<i64>().unwrap_or(0))
        .collect::<Vec<_>>();

    println!("Part 1 answer: {}", data.iter().sum::<i64>());
    let mut seen: HashSet<i64> = HashSet::new();

    let mut pos = 0;
    let mut val = 0;
    while !seen.contains(&val)  {
        seen.insert(val);
        pos += 1;
        val += data[pos % data.len()];
    }
    println!("Part 2: {}", val);
}