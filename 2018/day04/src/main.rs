use hashbrown::HashMap;

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let mut chronological = input.split('\n').collect::<Vec<_>>();
    chronological.sort();
    let mut start_time = 60;
    let mut guard = 0;
    let mut guards: HashMap::<usize, [usize; 60]> = HashMap::new();
    for line in chronological.iter() {
        let mut words = line.split(' ');
        words.next();
        let mut time = words.next().unwrap().split(':');
        let _hour = time.next().unwrap().parse::<usize>().unwrap();
        let minute =  time.next().unwrap()[0..2].parse::<usize>().unwrap();
        match words.next().unwrap() {
            "Guard" => {
                let guard_id = words.next().unwrap();
                guard = guard_id[1..].parse::<usize>().unwrap();
                start_time = 60;
            }
            "falls" => {
                start_time = minute;
            }
            "wakes" => {
                guards.entry(guard).or_insert([0;60]);
                for i in start_time..minute {
                    guards.get_mut(&guard).unwrap()[i] +=1;
                }
            }
            _ => unimplemented!()
        }
    }
    let mut max_sleep = 0;
    let mut sleepiest = 0;
    for guard in guards.keys() {
        let sleeping_time: usize = guards.get(&guard).unwrap().iter().sum();
        if sleeping_time > max_sleep {
            max_sleep = sleeping_time;
            sleepiest = *guard;
        }
    }
    println!("Part 1: {}", sleepiest * guards[&sleepiest]
                                        .iter()
                                        .position(|m| m == guards.get(&sleepiest)
                                                                    .unwrap()
                                                                    .iter()
                                                                    .max()
                                                                    .unwrap())
                                        .unwrap());
    let mut max_sleep = 0;
    let mut sleepiest = 0;
    for guard in guards.keys() {
        let sleeping_time: usize = *guards.get(&guard).unwrap().iter().max().unwrap();
        if sleeping_time > max_sleep {
            max_sleep = sleeping_time;
            sleepiest = *guard;
        }
    }
    println!("Part 2: {}", sleepiest * guards[&sleepiest]
                                        .iter()
                                        .position(|m| m == guards.get(&sleepiest)
                                                                    .unwrap()
                                                                    .iter()
                                                                    .max()
                                                                    .unwrap())
                                        .unwrap());
}
