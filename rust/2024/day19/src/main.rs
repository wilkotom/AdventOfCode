use std::{collections::{HashMap, HashSet}, error::Error};
use aochelpers::get_daily_input;

fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(19,2024)?;
    let (valid, designs) = parse_data(&data);
    let (part1, part2) = solve(&designs, &valid);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    Ok(())
}

fn solve(designs: &Vec<&str>, valid: &[&str]) -> (usize, usize) {
    let mut already_computed = HashMap::new();
    let max_pattern_len = valid.iter().map(|v| v.len()).max().unwrap();
    let part1 = designs.iter().filter(|p| count_valid_patterns(p, valid, &mut already_computed, max_pattern_len) > 0).count();
    // As we've built the cache, Part 2 is basically free
    let part2 = designs.iter().map(|p| count_valid_patterns(p, valid, &mut already_computed, max_pattern_len)).sum();
    (part1, part2)
}

fn count_valid_patterns<'a>(pattern: &'a str, valid: &[&'a str], already_computed: &mut HashMap<&'a str, usize>, max_len: usize) -> usize {
    let mut combinations = 0;
    if let Some(result) = already_computed.get(pattern) {
        return *result;
    }
    if pattern.is_empty() {
        return 1;
    }
    for i in 1..=max_len.min(pattern.len()) {
        if valid.contains(&&pattern[..i]) {
            let subcount = count_valid_patterns(&pattern[i..], valid, already_computed, max_len);
            combinations += subcount;
        }
    }
    already_computed.insert(pattern, combinations);
    combinations
}

fn parse_data(input: &str) -> (Vec<&str>, Vec<&str>) {
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
        assert_eq!(solve(&designs, &valid).0, 6);
    }

    #[test]
    fn test_part2() {
        let (valid, designs) = parse_data(TESTDATA);
        assert_eq!(solve(&designs, &valid).1, 16);
    }
}