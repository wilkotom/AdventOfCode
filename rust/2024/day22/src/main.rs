use std::{collections::{HashMap, HashSet}, error::Error};
use aochelpers::get_daily_input;

fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(22,2024)?;
    let secrets = data.lines().map(|v| v.parse::<isize>().unwrap()).collect::<Vec<_>>();
    println!("Part 1: {}", part1(&secrets));
    println!("Part 2: {}", part2(&secrets));
    Ok(())
}

fn part1(secrets: &[isize]) -> isize {
    let mut final_value = 0;
    for secret in secrets {
        let mut secret = *secret;
        (0..2000).for_each(|_| next_secret_number(&mut secret));
        final_value += secret
    }
    final_value
}

fn part2(secrets: &[isize]) -> isize {
    let mut sell_price_by_window= HashMap::new();
    for secret in secrets {
        let mut seen = HashSet::new();
        let mut prices = Vec::new();
        let mut deltas = Vec::new();
        let mut secret = *secret;
        prices.push(secret % 10);
        for i in 1..=2000 {
            next_secret_number(&mut secret);
            prices.push(secret % 10);
            deltas.push(secret % 10 - prices[i-1] );
        }
        for (i,window) in deltas.windows(4).enumerate()  {
            if !seen.contains(&window) {
                seen.insert(window);
                *sell_price_by_window.entry([window[0], window[1], window[2], window[3]]).or_insert(0) += prices[i+4]
            }
        }
    }
    *sell_price_by_window.values().max().unwrap()
}


fn next_secret_number(num: &mut isize) {
    
    *num ^= *num*64;
    *num %= 16777216;
    *num ^= *num/32;
    *num %= 16777216;
    *num ^= *num*2048;
    *num %= 16777216;

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let secrets = [1,10,100,2024];
        assert_eq!(part1(&secrets), 37327623);
    }

    #[test]
    fn test_part2() {
    }
}