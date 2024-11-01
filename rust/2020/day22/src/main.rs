use std::collections::VecDeque;
use std::collections::HashSet;


fn main() {
    let (player2, player1) = create_decks("./input.txt".to_string());
    println!("Part 1: {}", play_combat(player1.clone(), player2.clone()));
    let part_2_result = play_recursive_combat(player1.clone(), player2.clone());
    println!("Part 2: {}", if part_2_result.0 > 0 {part_2_result.0} else {part_2_result.1});

}

fn create_decks(filename: String) -> (VecDeque<usize>, VecDeque<usize>) {
    let mut players: Vec<VecDeque<usize>> = Vec::new();
    for player_details in std::fs::read_to_string(filename).unwrap_or("".to_string()).split("\n\n") {
        let cards = player_details.split(":\n").into_iter().last().unwrap().split("\n").map(|x| x.parse::<usize>().unwrap()).collect::<VecDeque<_>>();
        players.push(cards);
    }
    (players.pop().unwrap(), players.pop().unwrap())
}

fn play_combat(mut player1: VecDeque<usize>, mut player2: VecDeque<usize>) -> usize {
    while player1.len() != 0 && player2.len() != 0 {
        let p1 = player1.pop_front().unwrap();
        let p2 = player2.pop_front().unwrap();
        if p1 > p2 {
            player1.push_back(p1);
            player1.push_back(p2);
        } else {
            player2.push_back(p2);
            player2.push_back(p1);
        }
    }
    if player1.len() > 0 {
        score_hand(player1)
    } else {
        score_hand(player2)

    }
    
}

fn play_recursive_combat(mut player1: VecDeque<usize>, mut player2: VecDeque<usize>) -> (usize,usize) {
    let mut seen_hands: HashSet<(usize,usize)> = HashSet::new();
    while player1.len() != 0 && player2.len() != 0 {
         let scores = (score_hand(player1.clone()), score_hand(player2.clone()));
        if seen_hands.contains(&scores) {
            return (scores.0,0);
        } else  {
            seen_hands.insert(scores);
            let p1 = player1.pop_front().unwrap();
            let p2 = player2.pop_front().unwrap();
            if p1 <= player1.len() && p2 <= player2.len() {
                let mut sub_deck1 = player1.clone();
                sub_deck1.truncate(p1);
                let mut sub_deck2 = player2.clone();
                sub_deck2.truncate(p2);
                let result = play_recursive_combat(sub_deck1, sub_deck2);
                if result.0 > 0 {
                    player1.push_back(p1);
                    player1.push_back(p2);
    
                } else {
                    player2.push_back(p2);
                    player2.push_back(p1);
                }
            } else if p1 > p2 {
                player1.push_back(p1);
                player1.push_back(p2);
            } else {
                player2.push_back(p2);
                player2.push_back(p1);
            }
        // println!("{:?} {:?}", score_hand(player1.clone()), score_hand(player2.clone()));
        }
    }
    (score_hand(player1.clone()), score_hand(player2.clone()))
}

fn score_hand(mut hand: VecDeque<usize>) -> usize {
    let mut multiplier = 1;
    let mut score: usize = 0;
    while hand.len() > 0 {
        score += hand.pop_back().unwrap() * multiplier;
        multiplier += 1;
    }
    score
}

