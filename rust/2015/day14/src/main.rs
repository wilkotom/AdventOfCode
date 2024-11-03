use std::{fs::read_to_string, collections::HashMap};

#[derive(Debug,Clone)]
struct Reindeer {
    name: String,
    speed: i32,
    run_time: i32,
    rest_time: i32
}
fn main() {
    let data = read_to_string("./input.txt").unwrap();
    let mut reindeers: Vec<Reindeer> = Vec::new();
    for line in data.split('\n') {
        let words = line.split_ascii_whitespace().collect::<Vec<_>>();
        let name = words[0].to_owned();
        let speed = words[3].parse::<i32>().unwrap();
        let run_time = words[6].parse::<i32>().unwrap();
        let rest_time = words[13].parse::<i32>().unwrap();
        reindeers.push(Reindeer{name,speed,run_time,rest_time});
    }
    let period = 2503;
    let mut max_distance = 0;
    let mut scoreboard: HashMap<String, i32> = HashMap::new();
    for i in 1..period +1 {
        let mut distances: HashMap<i32,Vec<String>> = HashMap::new();
        for reindeer in &reindeers {
            let distance = (i / (reindeer.run_time + reindeer.rest_time)) * (reindeer.speed * reindeer.run_time) + 
                                (reindeer.run_time.min(i % (reindeer.run_time + reindeer.rest_time)) * reindeer.speed);
            max_distance= max_distance.max(distance);
            distances.entry(distance).or_default();
            distances.get_mut(&distance).unwrap().push(reindeer.name.clone());
        }
        let win_dist = distances.keys().max().unwrap();
        for winner in distances.get(win_dist).unwrap() {
            scoreboard.insert(winner.to_owned(), scoreboard.get(winner).unwrap_or(&0) + 1);
        }
    }
    println!("Part 1: {:?}", max_distance);
    println!("Part 2: {:?}", scoreboard.values().max().unwrap());
}
