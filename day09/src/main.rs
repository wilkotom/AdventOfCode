fn main() {
    let data = std::fs::read_to_string("./input.txt").unwrap().chars().collect::<Vec<_>>();
    let mut pointer = 0;
    let mut garbage = false;
    let mut score = 0;
    let mut nest = 0;
    let mut garbage_chars = 0;
    while pointer < data.len() {
        if data[pointer] == '!' && garbage {
            pointer +=1;
        } else if data[pointer] == '{' && ! garbage {
            nest +=1;
        } else if data[pointer] == '}' && ! garbage {
            score += nest;
            nest -= 1;
        } else if data[pointer] == '<' && ! garbage {
            garbage = true;
        } else if data[pointer] == '>' && garbage {
            garbage = false;
        } else if garbage {
            garbage_chars+=1;
        }
        pointer +=1;

    }

    println!("Part 1: {}", score);
    println!("Part 2: {}", garbage_chars);
}


