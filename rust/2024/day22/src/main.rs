use std::{collections::HashMap, error::Error};
use aochelpers::get_daily_input;

fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(22,2024)?;
    let secrets = data.lines().map(|v| v.parse::<isize>().unwrap()).collect::<Vec<_>>();
    //let secrets = [1,10,100,2024];
    //let results = part1(&secrets);
    // println!("{}", results.iter().sum::<usize>());

    println!("{}", part2(&secrets));
    Ok(())
}

fn part1(secrets: &[isize]) -> Vec<isize> {
    let mut answer = Vec::new();
    for secret in secrets {
        let mut secret = *secret;
        (0..2000).for_each(|_| next_secret_number(&mut secret));
        answer.push(secret)
    }
    answer
}

fn part2(secrets: &[isize]) -> isize {
    let mut prices_by_monkey: Vec<Vec<isize>> = Vec::new();
    let mut deltas_by_monkey: Vec<Vec<isize>> = Vec::new();
    for secret in secrets {
        let mut prices = Vec::new();
        let mut deltas = Vec::new();
        let mut secret = *secret;
        prices.push(secret % 10);
        for i in 1..=2000 {
            next_secret_number(&mut secret);
            prices.push(secret % 10);
            deltas.push(secret % 10 - prices[i-1] );
        }
        prices_by_monkey.push(prices);
        deltas_by_monkey.push(deltas);
    }

    let mut scores_for_sequence = HashMap::new();
    let mut best_score = 0;
    for monkey in deltas_by_monkey.iter() {
        for window in monkey.windows(4) {
            if scores_for_sequence.contains_key(window) {
                continue;
            }
            let mut score_for_tuple = 0;
            for (i, monkey) in deltas_by_monkey.iter().enumerate() {
                for (j,comparator_window) in monkey.windows(4).enumerate() {
                    if window == comparator_window {
                        score_for_tuple += prices_by_monkey[i][j+4];
                        break;
                    }
                }
            }
            if score_for_tuple > best_score {
                println!("New best: {:?} {}", window, score_for_tuple);
                best_score = score_for_tuple;
            }
            scores_for_sequence.insert(window, score_for_tuple);
        }
    }
    *scores_for_sequence.values().max().unwrap()
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
        assert_eq!(part1(&secrets), vec![8685429,4700978,15273692, 8667524]);
    }

    #[test]
    fn test_part2() {
    }
}