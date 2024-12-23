use std::{collections::HashMap, error::Error};

use aochelpers::get_daily_input;

#[derive(Debug,Hash,Eq,PartialEq, Copy, Clone)]
struct BingoCardSquare {
    marked: bool,
    location: Coordinate
}

#[derive(Debug,Hash,Eq,PartialEq, Copy, Clone)]
struct Coordinate {
    x: usize,
    y: usize
}

fn main() -> Result<(), Box<dyn Error>>{
    let input = get_daily_input(4, 2021)?;
    let split_data = input.split("\n\n").collect::<Vec<_>>();
    let bingo_numbers = split_data[0].split(",").map(|x| x.parse::<isize>().unwrap()).collect::<Vec<_>>();
    let mut cards: Vec<HashMap<isize,BingoCardSquare>> = Vec::new();

    for card in &split_data[1..] {
        let mut parsed_card: HashMap<isize, BingoCardSquare> = HashMap::new();
        for (y, line) in card.split("\n").enumerate() {
            for (x, number) in line.split_ascii_whitespace().map(|n| n.parse::<isize>().unwrap()).enumerate() {
                parsed_card.insert(number, BingoCardSquare{location: Coordinate{x,y}, marked: false});
            }
        }
        cards.push(parsed_card);
    }
    play_bingo(&mut cards, &bingo_numbers, true);
    Ok(())
}

fn play_bingo(cards: &mut Vec<HashMap<isize,BingoCardSquare>>, bingo_numbers: &Vec<isize>, first_winner: bool){
    let mut last_drawn_number: isize = -1;
    let mut score: isize = -1;
    let mut winning_card = usize::MAX;
    for number in bingo_numbers {
        last_drawn_number = *number;
        for card in cards.iter_mut() {
            if card.contains_key(&number) {
                card.get_mut(&number).unwrap().marked = true;
            }
        }
        for (i, card) in cards.iter().enumerate() {
            if is_winner(card) {
                winning_card = i;
                score = winning_score(&card);
            }
        }
        if winning_card != usize::MAX {
            break;
        }
    }
    if first_winner {
        println!("Part 1 answer: {}", last_drawn_number * score);
    }
    if cards.len() > 1 {
        cards.swap_remove(winning_card);
        play_bingo(cards, bingo_numbers, false)
    } else {
        println!("Part 2 answer: {}", last_drawn_number * score);

    } 
}

fn is_winner(card: &HashMap<isize,BingoCardSquare>) -> bool {
    let coord_mappings = card.values().map(|v| (v.location, v.marked)).collect::<HashMap<_,_>>();
    (0..5).map(|x| (0..5).map(|y| *coord_mappings.get(&Coordinate{x,y}).unwrap()).all(|v| v)).any(|v| v) ||  
    (0..5).map(|y| (0..5).map(|x| *coord_mappings.get(&Coordinate{x,y}).unwrap()).all(|v| v)).any(|v| v)

}

fn winning_score(card: &HashMap<isize,BingoCardSquare>) -> isize {
    let mut total: isize = 0;
    for (number, details) in card {
        if !details.marked {
            total += number;
        }
    }
    total
}