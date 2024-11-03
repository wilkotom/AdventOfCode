use std::collections::HashMap;
#[derive(Debug)]
struct Seen {
    offset: i128,
    timestamp: i128,
}
fn main() {

    /*
    
    This puzzle initially seemed to lend itself to bit masking; however I can't guarantee that all the 
    buckets fit into a space less than 128 bits, wide so rather than hack together something using 
    arbitrary precision numbers, I opted to just keep everything as a string.
    */

    let data = std::fs::read_to_string("./input.txt").unwrap();
    let mut sections = data.split("\n\n");
    let mut state = "....".to_owned();
    state.push_str(sections.next().unwrap().split(": ").last().unwrap());
    state.push_str("....");

    let mut mappings: HashMap<String,char> = HashMap::new();
    for mut mapping in sections.next().unwrap().split('\n').map(|s| s.split(" => ")) {
        mappings.insert(mapping.next().unwrap().to_owned(), mapping.next().unwrap().chars().next().unwrap());
    }

    let mut offset = 0;
    let mut timestamp = 0;
    let mut history : HashMap<String,Seen> = HashMap::new();


    loop {
        timestamp +=1;
        (state, offset) = generation(&state, offset, &mappings);
        if timestamp == 20 {
            let mut score = 0;
            for (i, c) in state[4..].chars().enumerate() {
                if c == '#' {
                    score += i as i128 + offset;
                }
            }
            println!("Part 1: {}", score);
        }
        if history.contains_key(&state) {
            break;
        } else {
            history.insert(state.clone(), Seen{timestamp,offset});
        }
    }

    /*
    Slight cheat here in that the my data converges to stability with a generation time of 1,
    ie. it does not cycle between different shapes. If it did not, would need instead to advance 
    the clock to the highest generation number less than fifty billion that matches the first repeat,
    then move forward a step at a time until it hits the fifty billionth generation.
    */

    let first_convergence = history.get(&state).unwrap();
    let mut score = 0;
    for (i, c) in state[4..].chars().enumerate() {
        if c == '#' {
            score += i as i128 + first_convergence.offset + 50000000000 - first_convergence.timestamp;
        }
    }

    println!("Part 2: {}", score);


}

fn generation(plants: &str, leftmost: i128, mappings: &HashMap<String,char>) -> (String, i128) {
    let mut next_gen = String::from("....");
    let mut start: Option<i128> = None;
    for i in 2..plants.len()-2 {
        let next_plant = *mappings.get(&plants[i-2..=i+2]).unwrap_or(&'.');
        next_gen.push(next_plant);
        if next_plant == '#' && start.is_none() {
            start = Some(i as i128 -4 + leftmost);
        }
    }
    next_gen.push_str("....");
    
    let first = next_gen.find('#').unwrap();
    let mut result = "....".to_owned();
    
    result.push_str(&next_gen[first..]);
    while &result[result.len()-5..] != "#...." {
        if result[result.len()-5..].contains('#') {
            result.push('.');
        }
        else {
            result.pop();
        }
    }
    (result, start.unwrap())
}
