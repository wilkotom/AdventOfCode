use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::collections::HashSet;
use std::cmp::Ordering;


fn main() -> Result<(), Error> {
    let mut numbers = read_nums(File::open("./input.txt")?)?;
    let mut number_set = HashSet::new();
    for num in numbers.iter() {
        number_set.insert(num);
        let sought = 2020 - num;
        if number_set.contains(&sought) {
            println!("Part 1 answer: {}", num * (sought));
        }
    }
    numbers.sort();

    for i in 0..(numbers.len() -1) {
        let mut left = i +1;
        let mut right = numbers.len() -1;
        if numbers[i] > 674 {
            break
        }
        while left < right {
            let total = numbers[i] + numbers[left] + numbers[right];
            match total.cmp(&2020){
                Ordering::Less => {
                    left += 1;
                },
                Ordering::Equal => {
                    println!("Part 2 Answer: {}", numbers[i]*numbers[left]*numbers[right]);
                    left += 1;
                    right -=1;
                },
                Ordering::Greater => {
                    right -= 1;
                },
            }
        }
    }
    

    Ok(())
}


fn read_nums<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    br.lines()
    .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}