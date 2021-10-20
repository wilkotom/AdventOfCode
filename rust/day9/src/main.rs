use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::collections::HashSet;


fn main() -> Result<(), Error> {
    let numbers = read_nums(File::open("./input.txt")?)?;
    let part1_answer = part1(&numbers, 25);
    println!("Part 1: {}", part1_answer);
    println!("Part 2: {}", part2(&numbers, part1_answer));
    Ok(())  
}


fn read_nums<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    br.lines()
    .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn get_valid_pairs(numbers: & [i64]) -> HashSet<i64> {
    let mut answer: HashSet<i64> = HashSet::new();
    for i in 0..(numbers.len() as usize -1) {
        for j in i+1..numbers.len() as usize{
            answer.insert(numbers[i] + numbers[j]);
        }
    }
    answer
}

fn part1(numbers: &Vec<i64>, window_size: usize) -> i64{
    let mut pointer = window_size;
    while get_valid_pairs(&numbers[pointer-window_size..pointer]).contains(&numbers[pointer]) {
        pointer += 1;
    }
    numbers[pointer]
}

fn part2(numbers: &Vec<i64>, target: i64) -> i64 {
    let mut left = 0;
    let mut right = numbers.len() -1;
    let mut total: i64 = numbers.iter().sum();
    while total != target {
        if total > target {
            total -= numbers[right];
            right -=1;
        } else {
            total -= numbers[left];
            total += numbers[right +1];
            left += 1;
            right +=1;
        }
    }
    numbers[left..right+1].iter().min().unwrap() + numbers[left..right+1].iter().max().unwrap()
}