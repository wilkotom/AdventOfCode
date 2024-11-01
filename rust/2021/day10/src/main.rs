use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let scores = HashMap::from([(')',3),(']', 57),('}', 1197), ('>', 25137), ('(',1),('[', 2), ('{', 3), ('<', 4)]);
    let opening = HashMap::from([(']','['), ('}','{'), ('>','<'), (')','(')]);
    let mut part1_score = 0;
    let mut part2_scores: Vec<i64> = Vec::new();
    for line in input.split('\n'){
        let mut stack : Vec<char> = Vec::new();
        let mut complete = true;
        let mut line_score = 0;
        for c in line.chars() {
            match c {
                '{' | '(' | '[' | '<' => {
                    stack.push(c)
                }
                _ => {
                    let last = stack.pop().unwrap();
                    if last != *opening.get(&c).unwrap() {
                        part1_score += scores.get(&c).unwrap();
                        complete = false;
                        break;
                    }
                }
            }
        }
        if complete {
            while !stack.is_empty() {
                let c = stack.pop().unwrap();
                line_score = (line_score * 5) + scores.get(&c).unwrap();
            }
            part2_scores.push(line_score);
        }
    }
    println!("Part 1 Score: {}", part1_score);
    part2_scores.sort_unstable();
    println!("Part 2 Winning Score: {}", part2_scores[part2_scores.len() / 2]);
}

