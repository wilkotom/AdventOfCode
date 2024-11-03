use std::collections::{HashMap,HashSet};

#[derive(Debug,Copy,Clone)]
struct Worker {
    job: Option<char>,
    finish: i32
}

fn main() {
    let mut parents: HashMap<char,Vec<char>> = HashMap::new();
    let mut all_steps: HashSet<char> = HashSet::new();
    let input = std::fs::read_to_string("./input.txt").unwrap();
    for line in input.split('\n') {
        let parent = line.chars().nth(5).unwrap();
        let child = line.chars().nth(36).unwrap();
        parents.entry(child)
            .and_modify(|p| p.push(parent))
            .or_insert(vec![parent]);
        all_steps.insert(parent);
        all_steps.insert(child);
    };
    let order = part1(&parents, all_steps.clone());
    println!("Part 1: {}",   order.iter().collect::<String>());
    println!("Part 2: {}",   part2(&parents, all_steps));
}

fn part1(parents: &HashMap<char,Vec<char>>, mut all_steps: HashSet<char>) -> Vec<char> {

    let mut ordering = Vec::new();
    while !all_steps.is_empty() {
        let mut sorted_steps = all_steps.iter().copied().collect::<Vec<_>>();
        sorted_steps.sort();
        for step in sorted_steps {
            let mut preqs_met = true;
            for prereq in  parents.get(&step).unwrap_or(&vec![]) {
                if !ordering.contains(prereq) {
                    preqs_met = false;
                }
            }
            if preqs_met {
                ordering.push(step);
                all_steps.remove(&step);
                break;
            }
        }
    }
    ordering
}

fn part2(parents: &HashMap<char,Vec<char>>, mut all_steps: HashSet<char>) -> i32 {
    let mut completed: HashSet<char> = HashSet::new();
    let mut time = 0;
    let mut workers = [Worker{job: None, finish: 0}; 5];
    while !all_steps.is_empty() {
        let mut assigned = false; 
        for next_step in all_steps.clone().iter() {
            for worker in workers.iter_mut() {
                if time >= worker.finish  && worker.job.is_some(){
                    completed.insert(worker.job.unwrap());
                    worker.job = None;
                }
            }
            if workers.iter().any(|w| w.job.is_none()) {
                let mut preqs_met = true;
                for prereq in  parents.get(next_step).unwrap_or(&vec![]) {
                    if !completed.contains(prereq) {
                        preqs_met = false;
                    }
                }
                if preqs_met {
                    for worker in workers.iter_mut() {
                        if worker.job.is_none() {
                            worker.job = Some(*next_step);
                            worker.finish = *next_step as i32 - 4 + time;
                            all_steps.remove(next_step);
                            assigned = true;
                            break;
                        }
                    }
                }
            } 
        }
        if !assigned {
            time += 1;
        }
    }
    workers.iter().map(|w| w.finish).max().unwrap()

}