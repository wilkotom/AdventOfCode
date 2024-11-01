use std::{error::Error, collections::HashMap, cmp::Ordering};
use aochelpers::get_daily_input;

fn main() -> Result<(), Box<dyn Error>>{
    let hands = build_bids(&get_daily_input(7,2023)?);
    println!("Part 1: {}", play_poker(&hands, |a,b| compare_hands(a, b, "23456789TJQKA")));
    println!("Part 2: {}", play_poker(&hands, |a,b| compare_hands(a, b, "J23456789TQKA")));
    Ok(())
}

fn play_poker(hands: &HashMap<String, i64>, compare_hands: fn(&&String, &&String) -> Ordering ) -> i64 {
    let mut hand_list: Vec<&String> = hands.keys().collect::<Vec<_>>();
    hand_list.sort_unstable_by(compare_hands);
    let mut total = 0;
    for (pos, hand) in hand_list.iter().enumerate() {
        total += (pos as i64 +1 ) * hands.get(*hand).unwrap();
    }
    total
}

fn build_bids(data: &str) -> HashMap<String, i64> {
    let mut hands = HashMap::new();
    for hand in data.split('\n'){
        let mut tokens = hand.split(' ');
        let cards = tokens.next().unwrap();
        let bid = tokens.next().unwrap().parse::<i64>().unwrap();
        hands.insert(cards.to_string(), bid);
    }
    hands
}

fn score_hand(hand: &str, jokers_wild: bool) -> i64 {
    let mut card_counts: HashMap<char,i64> = HashMap::new();
    for c in hand.chars() {
        *card_counts.entry(c).or_insert(0) += 1
    }
    if jokers_wild {
        if let Some(c) = card_counts.get(&'J') {
            if *c == 5 {
                return 7;
            }
            let mut card_to_upgrade = 'X';
            let mut most_cards = 0;
            for (k,v) in card_counts.iter() {
                if *k != 'J' && most_cards < *v {
                    card_to_upgrade = *k;
                    most_cards = *v;
                }
            }
            *card_counts.entry(card_to_upgrade).or_insert(0) += *c;
            card_counts.remove(&'J');
        }   
    }
    match card_counts.len() {
        1 => 7,
        2 => if card_counts.values().any(|c| *c == 4) {6} else {5},
        3 => if card_counts.values().any(|c| *c == 3) {4} else {3},
        4 => 2,
        5 => 1,
        _ => unimplemented!()
    }
}


fn compare_hands(left: &&String  , right: &&String, cards_order: &str ) -> Ordering {
    let jokers_wild = cards_order.starts_with('J');
    let left_score = score_hand(left, jokers_wild);
    let right_score = score_hand(right, jokers_wild);
    match left_score.cmp(&right_score) {
        Ordering::Less => Ordering::Less,
        Ordering::Equal => {
            let mut right_chars = right.chars();
            for lc in left.chars() {
                let rc = right_chars.next().unwrap();
                match cards_order.chars().position(|c| c == lc).cmp(&cards_order.chars().position(|c| c == rc)) {
                    Ordering::Less => {return Ordering::Less;}
                    Ordering::Greater => {return Ordering::Greater;}
                    _ => {}
                };
            }
            Ordering::Equal
        },
        Ordering::Greater => Ordering::Greater,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_score_part1() {
        assert_eq!(score_hand("32T3K", false), 2);
        assert_eq!(score_hand("T55J5", false), 4);
        assert_eq!(score_hand("KK677", false), 3);
        assert_eq!(score_hand("KTJJT", false), 3);
        assert_eq!(score_hand("QQQJA", false), 4);
        assert_eq!(score_hand("JJJJJ", false), 7);
    }

    #[test]
    fn test_part1() {
let hands = build_bids(DATA);
        assert_eq!(play_poker(&hands, |a,b| compare_hands(a, b, "23456789TJQKA")),6440);
    }

    #[test]
    fn test_part2() {
let hands = build_bids(DATA);
        assert_eq!(play_poker(&hands, |a,b| compare_hands(a, b, "J23456789TQKA")),5905);
    }
}
