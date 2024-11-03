use std::collections::{HashSet, VecDeque};
use std::{fs::File, collections::HashMap};
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    let mut mappings: HashMap<i32, HashSet<i32>> = HashMap::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines.map_while(Result::ok) {
            let mut sides = line.split(" <-> ");
            let source = sides.next().unwrap().parse::<i32>().unwrap();
            mappings.entry(source).or_default();
            for dest in sides.next().unwrap().split(", ") {
                mappings.get_mut(&source).unwrap().insert(dest.parse::<i32>().unwrap());
            }
        }
    }

    // println!("{:?}", mappings);

    let mut visited: HashSet<i32> = HashSet::new();
    let mut group_count = 0;
    for starting_point in mappings.keys(){
        if !visited.contains(starting_point) {
            let mut unvisited: VecDeque<i32> = VecDeque::new();
            unvisited.push_back(*starting_point);

            while ! unvisited.is_empty() {
                let candidate = unvisited.pop_front().unwrap();
                if ! visited.contains(&candidate) {
                    for next in &mappings[&candidate] {
                        unvisited.push_back(*next);
                    }
                    visited.insert(candidate);

                }
            }
            group_count += 1;
        }
    }
    println!("Part 2: {}", group_count);

}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}