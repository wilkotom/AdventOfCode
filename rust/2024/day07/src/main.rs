use std::{collections::HashSet, error::Error};
use aochelpers::get_daily_input;

#[derive(Debug,Clone)]
struct CalibrationState {
    target: i64,
    numbers: Vec<i64>,
}

fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(7,2024)?;
    let parsed: Vec<CalibrationState> = parse_data(&data);
    println!("Part 1: {}", solve(parsed, false));
    let parsed: Vec<CalibrationState> = parse_data(&data);
    println!("Part 2: {}", solve(parsed, true));

    Ok(())
}

fn solve(mut states: Vec<CalibrationState>, part2: bool) -> i64{
    let mut targets_met = HashSet::new();

    while let Some(mut state) = states.pop() {
        if state.numbers.len() == 1 {
            if state.numbers.pop().unwrap() == state.target {
                targets_met.insert(state.target);
            }
            continue;
        }
        let left = state.numbers.pop().unwrap();
        let right = state.numbers.pop().unwrap();

        let mut mul_state = state.clone();
        mul_state.numbers.push(left*right);
        states.push(mul_state);

        if part2 {
            let mut cat_state = state.clone();
            let mut cat_shift = 1;
            while ((cat_shift * left) + right) % cat_shift != right {
                cat_shift *=10
            }
            cat_state.numbers.push((cat_shift * left)+right);
            states.push(cat_state);
        }

        state.numbers.push(left+right);
        states.push(state);
    }
    targets_met.iter().sum()
}


fn parse_data (data: &str) -> Vec<CalibrationState>{
    let mut results = Vec::new();
    for line in data.lines() {
        let split = line.chars().position(|c |c ==':').unwrap();
        let target = line[..split].parse().unwrap();
        let numbers = line[split+1..].split_ascii_whitespace().map(|v| v.parse().unwrap()).rev().collect::<Vec<_>>();
        results.push(CalibrationState{
            target,
            numbers,
        });
    }

    results

}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part1() {
        let parsed: Vec<CalibrationState> = parse_data(TESTDATA);
        println!("{:?}", parsed);
        assert_eq!(solve(parsed, false), 3749)
    }

    #[test]
    fn test_part2() {
        let parsed: Vec<CalibrationState> = parse_data(TESTDATA);
        assert_eq!(solve(parsed, true), 11387)
    }
}