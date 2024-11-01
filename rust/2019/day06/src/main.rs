use std::collections::HashMap;

fn main() {
    let data = std::fs::read_to_string("./day06/input.txt").unwrap();
    let mut orbiting = HashMap::new();
    for line in data.split('\n') {
        let mut objects = line.split(')');
        let object = objects.next().unwrap().to_owned();
        let satellite = objects.next().unwrap().to_owned();
        orbiting.insert(satellite.to_owned(), object);
    }
    part1(&orbiting);
    part2(&orbiting);

}

fn part2(orbiting: &HashMap<String, String>) {
    let mut you_path = vec!["YOU"];
    let mut next = "YOU";
    while next != "COM" {
        next = &orbiting[next];
        you_path.push(next);
    }
    let mut san_path = vec!["SAN"];
    next = "SAN";
    while next != "COM" {
        next = &orbiting[next];
        san_path.push(next);
    }

    while you_path.pop() == san_path.pop() {
    }
    println!("Part 2: {}", you_path.len() + san_path.len());
}

fn part1(orbiting: &HashMap<String,String>) {
    let mut orbits_cache: HashMap<String, i32> = HashMap::new();
    orbits_cache.insert("COM".to_string(), 0);
    for object in orbiting.keys() {
        get_orbits(object, orbiting, &mut orbits_cache);
    }
    println!("Part 1: {}", orbits_cache.values().sum::<i32>());
   
}

fn get_orbits(object: &str, orbiting: &HashMap<String, String>, orbits_cache: &mut HashMap<String, i32>) -> i32 {
    if orbits_cache.contains_key(object) {
        *orbits_cache.get(object).unwrap()
    } else {
        let res= get_orbits(orbiting.get(object).unwrap(), orbiting, orbits_cache) +1;
        orbits_cache.insert(object.to_owned(), res);
        res
    }

}