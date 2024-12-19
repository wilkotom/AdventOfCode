use std::{collections::{HashMap, HashSet}, error::Error};
use aochelpers::get_daily_input;

fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(19,2024)?;
    let (valid, designs) = parse_data(&data);
    println!("Part 1: {}", part1(&designs, &valid));
    println!("Part 2: {}", part2(&designs, &valid));
    Ok(())
}

fn part1(designs: &Vec<&str>, valid: &Vec<&str>) -> usize {
    let mut already_computed = HashMap::new();
    let max_pattern_len = valid.iter().map(|v| v.len()).max().unwrap();
    designs.iter().filter(|p| count_valid_pattern(p, valid, &mut already_computed, max_pattern_len) > 0).count()
}

fn part2(designs: &Vec<&str>, valid: &Vec<&str>) -> usize {
    let mut already_computed = HashMap::new();
    let max_pattern_len = valid.iter().map(|v| v.len()).max().unwrap();
    designs.iter().map(|p| count_valid_pattern(p, valid, &mut already_computed, max_pattern_len)).sum()
}


fn count_valid_pattern<'a>(pattern: &'a str, valid: &Vec<&'a str>, already_computed: &mut HashMap<&'a str, usize>, max_len: usize) -> usize {
    let mut combinations = 0;
    if already_computed.contains_key(pattern) {
        return *already_computed.get(pattern).unwrap();
    }
    if pattern.is_empty() {
        return 1;
    }

    for i in 1..=max_len.min(pattern.len()) {
        if valid.contains(&&pattern[..i]) {
            let subcount = count_valid_pattern(&pattern[i..], valid, already_computed, max_len);
            combinations += subcount;
        }
    }
    already_computed.insert(pattern, combinations);
    combinations
}

fn parse_data<'a>(input: &'a str) -> (Vec<&'a str>, Vec<&'a str>) {
    let mut sections = input.split("\n\n");
    let valid = sections.next().unwrap().split(", ").collect();
    let designs = sections.next().unwrap().lines().collect();
    (valid, designs)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn test_part1() {
        let (valid, designs) = parse_data(TESTDATA);
        assert_eq!(part1(&designs, &valid), 6);
    }

    #[test]
    fn test_part2() {
        let (valid, designs) = parse_data(TESTDATA);
        assert_eq!(part2(&designs, &valid), 16);
    }
}