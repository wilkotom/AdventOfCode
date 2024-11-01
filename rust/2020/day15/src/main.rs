use std::fs;
use std::collections::HashMap;


fn main() {
    let raw_file = fs::read_to_string("./input.txt".to_string()).unwrap();
    let numbers: Vec<i64> = raw_file.split(",").map(|x| x.parse::<i64>().unwrap()).collect();
    let turns = 30000000;
    let mut spoken: HashMap<i64,i64> = HashMap::new();
    let mut last_spoken_number = 0;
    for (i, &v) in numbers.iter().enumerate() {
        spoken.insert(last_spoken_number, i  as i64);
        last_spoken_number = v;
    }
    if spoken.get(&0).unwrap_or(&0) == &0 {
        spoken.remove(&0);
    }
    let mut turn: i64 = numbers.len() as i64  + 1;
    while turn <= turns {
        last_spoken_number =  match spoken.get(&last_spoken_number){
            Some(&prev_spoken) => {
                spoken.insert(last_spoken_number, turn -1);
                turn - 1 - prev_spoken
            },
            None => {
                spoken.insert(last_spoken_number, turn -1);
                0
            }
        };
        turn += 1;
    }
    println!("Last Spoken Number: {}", last_spoken_number)
}
