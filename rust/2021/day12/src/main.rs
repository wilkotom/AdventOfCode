use std::collections::{HashMap, HashSet};


fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let mut routes: HashMap<String, Vec<String>> = HashMap::new();
    for line in input.split('\n') {
        let mut dests = line.split('-');
        let a = String::from(dests.next().unwrap());
        let b = String::from(dests.next().unwrap());
        if !routes.contains_key(&a) {
            routes.insert(a.clone(), Vec::new());
        }
        if !routes.contains_key(&b) {
            routes.insert(b.clone(), Vec::new());
        }
        routes.get_mut(&a).unwrap().push(b.to_owned());
        routes.get_mut(&b).unwrap().push(a);
    }
    let valid_paths_part_1 = find_paths(vec!(String::from("start")), &routes, false);
    println!("Part 1: {:?}", valid_paths_part_1.len());

    let valid_paths_part_2 = find_paths(vec!(String::from("start")), &routes, true);
    println!("Part 2: {:?}", valid_paths_part_2.len());
}

fn find_paths(visited: Vec<String>, routes: &HashMap<String, Vec<String>>, part2:bool ) -> Vec<Vec<String>> {
    let last = visited.last().unwrap();
    if last  == "end" {
        vec![visited]
    } else {
        let mut results: Vec<Vec<String>> = Vec::new();
        // if we've seen two identical small caves before, recurse as if we were running part 1
        let part2 = part2 && !contains_two_identical_lowers(&visited);
        for next_dest in routes.get(last).unwrap() {
            // never visit start more than once
            if next_dest != "start" && (!visited.contains(next_dest) || next_dest.to_uppercase() == *next_dest || part2) {
                let mut next_visited = visited.clone();
                next_visited.push(next_dest.clone());
                results.append(&mut find_paths(next_visited, routes, part2));
            }
        }
        results
    }
}

fn contains_two_identical_lowers( visited: &[String]) -> bool {
    let mut seen: HashSet<String> = HashSet::new();
    for place in visited {
        if place.to_uppercase() != *place {
            if seen.contains(place) {
                return true;
            }
           seen.insert(place.clone());
        }
    }
    false
}