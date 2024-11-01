use std::{fs::read_to_string, collections::HashMap};
use itertools::Itertools;

fn main() {
    let data = read_to_string("./input.txt").unwrap();

    let mut preferences: HashMap<String,HashMap<String,i32>> = HashMap::new();
    for line in data.split('\n') {
        let details = line.split_ascii_whitespace().collect::<Vec<_>>();
        preferences.entry(details[0].to_owned()).or_insert_with(HashMap::new);
        let difference = details[3].parse::<i32>().unwrap() * if details[2] == "gain" {1} else {-1};
        let person = &details[10][..details[10].len()-1];
        preferences.get_mut(details[0]).unwrap().insert(person.to_owned(), difference);
    }

    println!("Part 1: {}", get_max_happiness(&preferences));
    
    preferences.entry("Me".to_owned()).or_insert_with(HashMap::new);
    println!("Part 2: {}", get_max_happiness(&preferences));
}

fn get_max_happiness(preferences : &HashMap<String,HashMap<String,i32>>) -> i32 {
    let mut max_happiness = 0;
    let peeps = preferences.keys().map(|x| x.to_owned()).collect_vec();
    for mut permutation in peeps[1..].iter().permutations(peeps.len() -1) {
        permutation.push(&peeps[0]);

        let mut happiness = 0;
        for (i, person) in permutation.iter().enumerate() {
            let left = *preferences.get(*person).unwrap().get(permutation[(i + permutation.len() -1 ) % permutation.len()]).unwrap_or(&0);
            let right = *preferences.get(*person).unwrap().get(permutation[(i+1) % permutation.len()]).unwrap_or(&0);
            happiness += left + right;
        }
        max_happiness = max_happiness.max(happiness);
    }
    max_happiness

}