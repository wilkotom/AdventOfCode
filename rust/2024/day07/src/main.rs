use std::error::Error;
use aochelpers::get_daily_input;

#[derive(Debug)]
struct CalibrationState {
    target: i64,
    numbers: Vec<i64>,
}

fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(7,2024)?;
    let parsed: Vec<CalibrationState> = parse_data(&data);
    println!("Part 1: {}", solve(&parsed, false));
    println!("Part 2: {}", solve(&parsed, true));
    Ok(())
}

fn solve(states: &[CalibrationState], part2: bool) -> i64{
    let mut answer = 0;
    let mut unprocessed = Vec::new();
    'outer: for state in states {
        unprocessed.clear();
        unprocessed.push((state.numbers[0],1));
        while let Some((accumulator, pointer)) = unprocessed.pop(){
            if pointer == state.numbers.len() {
                if accumulator == state.target {
                    answer += state.target;
                    continue 'outer;
                }
                continue;
            }
            if accumulator > state.target {
                continue;
            }
            unprocessed.push((accumulator + state.numbers[pointer], pointer+1));
            unprocessed.push((accumulator * state.numbers[pointer], pointer+1));
            if part2 {
                let mut cat_shift = 10;
                while ((cat_shift * accumulator) + state.numbers[pointer]) % cat_shift != state.numbers[pointer] {
                    cat_shift *=10
                }
                unprocessed.push((accumulator * cat_shift + state.numbers[pointer], pointer+1));
            }
        }
    }
    answer
}

fn parse_data (data: &str) -> Vec<CalibrationState>{
    let mut results = Vec::new();
    for line in data.lines() {
        let split = line.chars().position(|c |c ==':').unwrap();
        let target = line[..split].parse().unwrap();
        let numbers = line[split+1..].split_ascii_whitespace().flat_map(|v| v.parse()).collect::<Vec<_>>();
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
        assert_eq!(solve(&parsed, false), 3749)
    }

    #[test]
    fn test_part2() {
        let parsed: Vec<CalibrationState> = parse_data(TESTDATA);
        assert_eq!(solve(&parsed, true), 11387)
    }
}