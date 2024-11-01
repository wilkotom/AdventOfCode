use std::collections::{VecDeque, HashMap};

fn main() {
    println!("{}",play_game(100, 2140));
}


fn play_game(players: usize, turns: usize) -> usize {

    let mut circle = VecDeque::from([0]);
    let mut scores: HashMap<usize,usize> = HashMap::new();
    let mut turn = 1;
    while turn <= turns {
        if turn % 23 != 0 {
            circle.push_back(turn);
            circle.rotate_left(1);
        } else {
            circle.rotate_right(8);
            let player_score = circle.pop_back().unwrap() + turn;
            scores.insert(turn % players, *scores.get(&(turn % players)).unwrap_or(&0) + player_score);
            circle.rotate_left(2);
        }
        turn +=1;
    }
    *scores.values().max().unwrap_or(&0)
}


#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn test_9_25() {
        assert_eq!(play_game(9,25), 32);
    }


    #[test]
    fn test_10_1618() {
        assert_eq!(play_game(10,1618), 8317);
    }

    #[test]
    fn test_13_7999() {
        assert_eq!(play_game(13,7999), 146373);
    }

    #[test]
    fn test_17_1104() {
        assert_eq!(play_game(17,1104), 2764);
    }

    #[test]
    fn test_21_6111() {
        assert_eq!(play_game(21,6111), 54718);
    }

    #[test]
    fn test_30_7999() {
        assert_eq!(play_game(30,5807), 37305);
    }
}