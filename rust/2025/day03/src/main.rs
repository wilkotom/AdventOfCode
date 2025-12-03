use aochelpers::get_daily_input;
use std::{collections::HashMap, error::Error};


fn main() -> Result<(), Box<dyn Error>> {
    let input = get_daily_input(3, 2025)?;
    let mut cache = HashMap::new();
    let jolts = input.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<_>>()).collect::<Vec<_>>();
    println!("Part 1: {}", jolts.iter().map(|j| biggest(j, 2, &mut cache)).sum::<usize>());
    println!("Part 2: {}", jolts.iter().map(|j| biggest(j, 12, &mut cache)).sum::<usize>());

    Ok(())
}

fn biggest<'a>(batteries: &'a [usize], s: usize, cache: &mut HashMap<(&'a [usize], usize),usize>) -> usize{
    if let Some(&answer) = cache.get(&(batteries, s)) {
        answer
    } else if s > batteries.len() {
        0
    } else if s == 2 {
        let answer = batteries[..batteries.len() -1].iter().enumerate().map(|(i,j)|10 *j + batteries[i+1..].iter().max().unwrap()).max().unwrap_or_default();
        cache.insert((batteries, 2), answer);
        answer
    } else {
        let answer = (batteries[0] * 10_usize.pow(s as u32 -1) + biggest(&batteries[1..], s -1, cache)).max(biggest(&batteries[1..], s, cache));
        cache.insert((batteries, s), answer);
        answer
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_p1() {
        let jolts = TESTDATA.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<_>>()).collect::<Vec<_>>();
        assert_eq!(jolts.iter().map(|j| biggest(j, 2, &mut HashMap::new())).sum::<usize>(), 357)
    }

    #[test]
    fn test_p2() {
        let jolts = TESTDATA.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<_>>()).collect::<Vec<_>>();
        assert_eq!(biggest(&[1,1,1], 3, &mut HashMap::new()),111);
        assert_eq!(biggest(&[1,1,1], 4, &mut HashMap::new()),0);

        assert_eq!(biggest(&[2, 1, 1, 1, 1, 1, 1, 1], 7, &mut HashMap::new()), 2111111);
        assert_eq!(biggest(&jolts[0], 12, &mut HashMap::new()), 987654321111);
        assert_eq!(biggest(&jolts[1], 12, &mut HashMap::new()), 811111111119);
        assert_eq!(biggest(&jolts[2], 12, &mut HashMap::new()), 434234234278);
        assert_eq!(biggest(&jolts[3], 12, &mut HashMap::new()), 888911112111);
    }
}