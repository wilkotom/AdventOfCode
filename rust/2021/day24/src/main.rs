use hashbrown::HashMap;
use cached::proc_macro::cached;

fn main() {
    let data = std::fs::read_to_string("./input.txt").unwrap();
    let pairs = input_to_rules(&data);
    let mut candidates = vec!["".to_string()];
    'outer: loop {
        let mut next_prefixes: Vec<String> = Vec::new();
        for prefix in &candidates {
            for c in '1'..':' {
                let mut candidate = prefix.clone();
                candidate.push(c);
                if candidate.len() > 14 {
                    break 'outer;
                }
                if evaluate_rules(candidate.clone(), pairs.clone()).is_ok() {
                    next_prefixes.push(candidate)
                }
            }
        }
        candidates = next_prefixes;
    }
    println!("Part 1: {}", candidates[candidates.len()-1]);
    println!("Part 2: {}", candidates[0]);
}

fn input_to_rules (input: &str) -> Vec<(i32,i32)> {
    let mut sections =  input.split("inp w");
    let _ = sections.next();

    let mut pairs: Vec<(i32,i32)> = Vec::new();
    for section in sections {
        let lines = section.split('\n').collect::<Vec<_>>();
        let a = lines[5].split_ascii_whitespace().nth(2).unwrap().parse::<i32>().unwrap();
        let b = lines[15].split_ascii_whitespace().nth(2).unwrap().parse::<i32>().unwrap();
        pairs.push((a,b));
    }

    println!("{:?}", pairs);
    pairs
}

#[cached]
fn evaluate_rules(input_number: String,rule_pairs: Vec<(i32,i32)>) -> Result<i32, i32> {
    if input_number.is_empty() {
        Ok(0)
    } else {
        let position = input_number.len() -1;
        let num = input_number.chars().nth(position).unwrap().to_digit(10).unwrap() as i32;
        if let Ok(z) = evaluate_rules(String::from(&input_number[0..position]), rule_pairs.clone()) {
            let (a,b) = rule_pairs[position];
            if a > 0 {
                let z = (z *26) + (num+b);
                Ok(z)
            } else {
                let comp = z % 26;
                let z = z / 26;
                if comp + a  != num { 
                    Err(z)
                } else {
                    Ok(z)
                }
            }

        } else {
            Err(0)
        }
    }

    
}