use aochelpers::get_daily_input;
use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(5, 2025)?;
    let (ranges, ingredients) = parse_data(&data);
    println!("Part 1: {}", part1(&ranges, &ingredients));
    println!("Part 2: {}", part2(ranges));

    Ok(())
}

fn part1(ranges: &[(u64, u64)], ingredients: &[u64]) -> usize {
    ingredients.iter().filter(|i| ranges.iter().any(|(mn, mx)| i >= &mn && i <= &mx)).count()
}

fn part2(mut ranges: Vec<(u64, u64)>) -> u64 {
    if ranges.is_empty() {
        return  0;
    }
    let mut result = 0;
    ranges.sort_unstable();
    let mut active_range = ranges[0];

    for range in &ranges[1..] {
        if range.0 > active_range.1 {
            result += active_range.1 - active_range.0 +1;
            active_range = *range;
        } else {
            active_range.1 = active_range.1.max(range.1)
        }
    }
    result + active_range.1 - active_range.0 +1
}

fn parse_data(data: &str) -> (Vec<(u64,u64)>, Vec<u64>) {
    let mut sections = data.split("\n\n");
    let ranges = sections.next().unwrap().lines().map(|l| {
        let split = l.find('-').unwrap(); 
        (l[..split].parse().unwrap(), l[split+1..].parse().unwrap())
    } ).collect();
    let ingredients = sections.next().unwrap().lines().map(|l| l.parse().unwrap()).collect();
    (ranges, ingredients)
}

#[cfg(test)]
mod tests {

    use super::*;

    const TESTDATA: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_p1() {
        let (ranges, ingredients) = parse_data(TESTDATA);
        assert_eq!(part1(&ranges, &ingredients), 3);
    }

    #[test]
    fn test_p2() {
        let (ranges, _) = parse_data(TESTDATA);
        assert_eq!(part2(ranges), 14);
    }
    
}