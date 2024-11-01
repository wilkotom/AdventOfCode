use std::fs::read_to_string;
use hashbrown::{HashSet,HashMap};

fn main() {
    let mut part1_lights: HashSet<(usize,usize)> = HashSet::new();
    let mut part2_lights: HashMap<(usize,usize), usize> = HashMap::new();
    let data = read_to_string("./input.txt").unwrap();
    for line in data.split('\n') {
        let mut tokens = line.split_ascii_whitespace();
        let mut mode = tokens.next();
        if mode == Some("turn") {
            mode = tokens.next();
        }
        let mut start = tokens.next().unwrap().split(',');
        tokens.next();
        let mut finish = tokens.next().unwrap().split(',');

        let start_x = start.next().unwrap().parse::<usize>().unwrap();
        let end_x = finish.next().unwrap().parse::<usize>().unwrap();
        let start_y = start.next().unwrap().parse::<usize>().unwrap();
        let end_y = finish.next().unwrap().parse::<usize>().unwrap();

        for x in start_x..end_x+1 {
            for y in start_y..end_y+1 {
                match mode {
                    Some("on") => {
                        part1_lights.insert((x,y));
                        part2_lights.insert((x,y), part2_lights.get(&(x,y)).unwrap_or(&0) +1);
                    },
                    Some("off") => {
                        part1_lights.remove(&(x,y));
                        let part2_brightness = *part2_lights.get(&(x,y)).unwrap_or(&1);
                        if part2_brightness == 1 {
                            part2_lights.remove(&(x,y));
                        } else {
                            part2_lights.insert((x,y), part2_brightness - 1);
                        }
                    },
                    Some("toggle") => {
                        if part1_lights.contains(&(x,y)) {
                            part1_lights.remove(&(x,y));
                        } else {
                            part1_lights.insert((x,y));
                        }
                        part2_lights.insert((x,y), part2_lights.get(&(x,y)).unwrap_or(&0) +2);
                    }
                    _ => {}
                }
            }
        }
    }
    println!("{}", part1_lights.len());
    println!("{}", part2_lights.values().sum::<usize>());
}
