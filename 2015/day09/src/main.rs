use std::collections::HashMap;
use itertools::Itertools;


fn main() {
    let data = std::fs::read_to_string("./input.txt").unwrap();
    let mut distance_mappings: HashMap<String,HashMap<String,i32>> = HashMap::new();

    for line in data.split('\n') {
        let mut words = line.split_ascii_whitespace();
        let left = words.next().unwrap().to_owned();
        words.next();
        let right = words.next().unwrap().to_owned();
        words.next();
        let distance = words.next().unwrap().parse::<i32>().unwrap();
        distance_mappings.entry(left.clone()).or_insert_with(HashMap::new);
        distance_mappings.entry(right.clone()).or_insert_with(HashMap::new);
        distance_mappings.get_mut(&left).unwrap().insert(right.clone(), distance);
        distance_mappings.get_mut(&right).unwrap().insert(left.clone(), distance);
    }

    let mut cache: HashMap<Vec<String>, i32> = HashMap::new();
    let mut shortest = i32::MAX;
    let mut longest = 0;

    for perm in distance_mappings.keys().cloned().permutations(distance_mappings.len()) {
        let res = get_distance(perm.clone(), &distance_mappings, &mut cache);
        shortest = shortest.min(res);
        longest = longest.max(res);

    }
    println!("Part 1: {}", shortest);
    println!("Part 2: {}", longest);
}

fn get_distance(route:Vec<String>, distance_mappings: &HashMap<String,HashMap<String,i32>>, cache: &mut HashMap<Vec<String>, i32>) -> i32 {
    if cache.contains_key(&route) {
        cache[&route]
    } else {
        let result = if route.len() > 2 {
            distance_mappings.get(&route[0]).unwrap().get(&route[1]).unwrap() + get_distance((&route[1..]).to_vec(), distance_mappings, cache) 
        } else {
            *distance_mappings.get(&route[0]).unwrap().get(&route[1]).unwrap()
        };
        cache.insert(route, result);
        result
    }
}