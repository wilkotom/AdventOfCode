
use cached::proc_macro::cached;

#[derive(Debug,Copy,Clone)]
struct Die {
    face: i64,
    roll_count: i64
}

impl Die {
    fn roll(&mut self) -> i64 {
        self.face %= 100;
        self.face += 1;
        self.roll_count +=1;
        self.face
    }
}

#[derive(Debug,Copy,Clone, Hash,PartialEq, Eq)]
struct Player {
    square: i64,
    score: i64
}

fn main() {
    let player1 = Player{square: 7, score:0};
    let player2 = Player{square: 5, score:0};
    println!("Part 1: {}", part1(player1, player2));
    let part2_res = part2(player1, player2);
    println!("Part 2: {}", part2_res.0.max(part2_res.1));

}

fn part1(player1: Player, player2: Player) -> i64{
    let mut die  = Die{face:100, roll_count:0};
    let mut players: Vec<Player> = vec![player1, player2];
    'game: loop {
        for player in players.iter_mut() {
            let forward = die.roll() + die.roll() + die.roll();
            // println!("Move: {}", forward);
            player.square = (player.square + forward -1 ) % 10 + 1;
            player.score += player.square;
            // println!("{:?}", player);
            if player.score >= 1000 {
                break 'game;
            }
        }

    }
    players[0].score.min(players[1].score) * die.roll_count
}

#[cached]
fn part2(player1: Player, player2: Player) -> (i128, i128) {
    if player2.score >= 21 {
       (0,1)    
   } else {
       // Pairs of number of squares to move forward and the number of different dice combinations that result in that distance
       let die_results = [(3,1), (4,3), (5,6), (6,7), (7,6), (8,3), (9,1)];
       let mut player1_wins = 0;
       let mut player2_wins = 0;
       for (forward, freq) in die_results {
           let next_square = (player1.square + forward -1 ) % 10 + 1;
           let next_score = player1.score + next_square;
           let (player2_quantum, player1_quantum) = part2(player2, Player{square: next_square, score: next_score});
           player1_wins += player1_quantum * freq;
           player2_wins += player2_quantum * freq;
       } 
       (player1_wins, player2_wins)
   }
}