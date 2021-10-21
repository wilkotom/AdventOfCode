use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::collections::HashMap;


fn main() -> Result<(), Error> {
    let mut jolts = read_nums(File::open("./input.txt")?)?;
    jolts.sort();
    println!("Part 1: {:?}", part1(&jolts));
    part2(&jolts);
    println!("Part 2: {}", part2(&jolts));
    Ok(())  
}

fn part1(jolts: &Vec<i64>) ->  i64 {
    let mut ones = 1;
    let mut threes = 1;
    for i in 1..jolts.len() {
        if jolts[i] - jolts[i-1] == 3 {
            threes += 1;
        } else {
            ones += 1;
        }
    }
    ones * threes
}

fn part2(jolts: &Vec<i64>) -> i64 {
    let mut steps: HashMap<i64, i64> = HashMap::new();
    steps.insert(0,1);
    for i in jolts.iter() {
        steps.insert(*i, steps.get(&(i-1)).unwrap_or(&(0)) + 
                         steps.get(&(i-2)).unwrap_or(&(0)) + 
                         steps.get(&(i-3)).unwrap_or(&(0)));
    }
    steps[jolts.iter().last().unwrap()]
}

fn read_nums<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    br.lines()
    .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}