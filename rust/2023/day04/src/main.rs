use std::error::Error;
use aochelpers::get_daily_input;

fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(4,2023)?;
    println!("Results: {:?}", part1(&data));
    Ok(())
}

fn play_scratchcard(line: &str) -> u32 {
    let mut parts = line.split(" | ");
    let my_card = parts.next().unwrap().split(' ').filter_map(|n| n.parse::<i32>().ok()).collect::<Vec<_>>();
    let winning_numbers = parts.next().unwrap().split(' ').filter_map(|n| n.parse::<i32>().ok()).collect::<Vec<_>>();
    my_card.iter().filter(|n| winning_numbers.contains(n)).count() as u32
}

fn part1(data: &str) -> (i32,i32){
    let mut result = 0;
    let cards = data.split('\n').collect::<Vec<_>>();
    let mut card_counts = vec![1; cards.len()];
    for (index, line) in cards.iter().enumerate() {
        let score = play_scratchcard(line);
        if score > 0 {
            result += i32::pow(2, score -1);
            for x in index+1..=(index+score as usize) {
                card_counts[x] += card_counts[index] 
            }
        }
    }
    (result, card_counts.iter().sum::<i32>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play_game1() {
        assert_eq!(play_scratchcard("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"), 4);
        assert_eq!(play_scratchcard("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"), 2);
        assert_eq!(play_scratchcard("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"), 2);
        assert_eq!(play_scratchcard("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"), 1);
        assert_eq!(play_scratchcard("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"), 0);
        assert_eq!(play_scratchcard("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"), 0);
    }
    #[test]
    fn test_all_parts() {
        assert_eq!(part1("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"), (13,30))
    }
}