

fn main() {
    let input: usize = 894501;
    part1(&input);
    part2(input)
}

fn part1(reps: &usize) {
    let mut scoreboard = vec![3,7];
    let mut pos1 = 0;
    let mut pos2 = 1;
    for _ in 0..*reps {
        let mut next_score = scoreboard[pos1] + scoreboard[pos2];
        if next_score >= 10 {
            scoreboard.push(next_score / 10);
            next_score %= 10;
        } 
        scoreboard.push(next_score);
        
        pos1 = (pos1 + scoreboard[pos1] +1) % scoreboard.len();
        pos2 = (pos2 + scoreboard[pos2] +1) % scoreboard.len();
    }
    print!("Part 1: ");
    for i in *reps..*reps+10 {
        print!("{}", scoreboard[ i % scoreboard.len()]);
    }
    println!()
}

fn part2(mut target: usize) {
    let mut target_seq = vec![];
    while target > 0{
        target_seq.insert(0, target % 10);
        target /= 10;
    }
    let mut target_seq_pos = 0;
    let mut scoreboard = vec![3,7];
    let mut pos1 = 0;
    let mut pos2 = 1;

    while target_seq_pos < target_seq.len() {
        let mut next_score = scoreboard[pos1] + scoreboard[pos2];
        if next_score >= 10 {
            let next_val = next_score / 10;
            if target_seq[target_seq_pos] == next_val {
                target_seq_pos +=1;
                if target_seq_pos == target_seq.len() {
                    break;
                }
            } else if target_seq[0] == next_val {
                target_seq_pos = 1;
            } else {
                target_seq_pos = 0;
            }
            scoreboard.push(next_val);
            next_score %= 10;
        }

        if target_seq[target_seq_pos] == next_score {
            target_seq_pos +=1;
            if target_seq_pos == target_seq.len() {
                break;
            }
        } else if target_seq[0] == next_score {
            target_seq_pos = 1;
        } else {
            target_seq_pos = 0;
        }
        scoreboard.push(next_score);
       
        pos1 = (pos1 + scoreboard[pos1] +1) % scoreboard.len();
        pos2 = (pos2 + scoreboard[pos2] +1) % scoreboard.len();

    }
    println!("Part 2: {}", scoreboard.len() - target_seq.len() +1);
}
