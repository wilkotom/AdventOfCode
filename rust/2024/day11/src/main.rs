use std::{collections::HashMap, error::Error};
use aochelpers::get_daily_input;

fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(11,2024)?;
    let pebbles = data.split_ascii_whitespace().map(|v| v.parse().unwrap()).collect::<Vec<_>>();
    println!("Part 1: {}", solve(&pebbles, 25));
    println!("Part 2: {}", solve (&pebbles, 75));

    Ok(())
}

fn solve(pebbles: &[i64], rounds: usize) -> i64{
    let mut pebble_counts: HashMap<i64,i64> = HashMap::new();
    pebbles.iter().for_each(|p| *pebble_counts.entry(*p).or_default() +=1);
    for _ in 0..rounds {
        let mut next_counts: HashMap<i64,i64> = HashMap::new();
        for (pebble, count) in pebble_counts {
            match blink(&pebble) {
                (left, None) => {           
                    *next_counts.entry(left).or_default() += count;
                }
                (left, Some(right)) => {
                    *next_counts.entry(left).or_default() += count;
                    *next_counts.entry(right).or_default() += count;
                }
            }
        }
        pebble_counts = next_counts
    }
    pebble_counts.values().sum()
}

fn blink(pebble: &i64) -> (i64, Option<i64>){
    match pebble {
        0 => (1, None),
        n if n.checked_ilog10().unwrap_or(0) %2 == 1 => {
            let divisor=  10_i64.pow(n.checked_ilog10().unwrap_or(0)/2 +1);
            (pebble / divisor, Some(pebble % divisor))
        }
        _ => (pebble * 2024, None)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blink() {
        assert_eq!(blink(&0), (1, None));
        assert_eq!(blink(&1), (2024, None));
        assert_eq!(blink(&12), (1, Some(2)));
        assert_eq!(blink(&1001), (10, Some(1)));
        assert_eq!(blink(&123456), (123, Some(456)));
    }

    #[test]
    fn test_solution() {
        let pebbles = "125 17".split_ascii_whitespace().map(|v| v.parse().unwrap()).collect::<Vec<_>>();
        assert_eq!(solve(&pebbles, 5), 13);
        assert_eq!(solve(&pebbles, 6), 22);
        assert_eq!(solve(&pebbles, 25), 55312);

    }
}
