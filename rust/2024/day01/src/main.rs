#![feature(binary_heap_drain_sorted)]

use std::{collections::{BinaryHeap, HashMap}, error::Error};
use aochelpers::get_daily_input;

fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(1,2024)?;
    let (col_a, col_b) = parse_data(&data);

    println!("Part 1: {}", part1(&col_a, &col_b));
    println!("Part 2: {}", part2(&col_a, &col_b));

    Ok(())
}

fn parse_data(input: &str) -> (Vec<i64>, Vec<i64>) {
    let mut col_a = Vec::new();
    let mut col_b = Vec::new();
    for line in input.lines() {
        let mut nums = line.split_ascii_whitespace();
        col_a.push(nums.next().unwrap().parse::<i64>().unwrap());
        col_b.push(nums.next().unwrap().parse::<i64>().unwrap());
    }
    (col_a, col_b)
}

fn part1(col_a: &[i64], col_b: &[i64]) -> i64 {
    
    let mut nums_a = col_a.iter().collect::<BinaryHeap<_>>();
    let mut nums_b = col_b.iter().collect::<BinaryHeap<_>>();
    nums_a.drain_sorted().map(|a| (a - nums_b.pop().unwrap()).abs()).sum()

}

fn part2(col_a: &[i64], col_b: &[i64]) -> i64 {
    let mut b_counts:HashMap<&i64, i64> = HashMap::new();
    col_b.iter().for_each(|b| *b_counts.entry(b).or_default() +=1);
    col_a.iter().map(|a| a*  b_counts.get(a).unwrap_or(&0)).sum()

}


#[cfg(test)]
mod tests {

    const P1TESTDATA: &str ="3   4
4   3
2   5
1   3
3   9
3   3";
    use super::*;

    #[test]
    fn test_part1() {
        let (col_a, col_b) = parse_data(P1TESTDATA);
        assert_eq!(col_a, vec![3,4,2,1,3,3]);
        assert_eq!(col_b, vec![4,3,5,3,9,3]);
        assert_eq!(part1(&col_a, &col_b), 11);
        assert_eq!(part1(&col_b, &col_a), 11);

    }

    #[test]
    fn test_part2() {
        let (col_a, col_b) = parse_data(P1TESTDATA);
        assert_eq!(part2(&col_b, &col_a), 31);

    }
}
