use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let mut counter = 0;
    let mut total = 0;
    for line in input.split('\n') {
        let mut number_mappings: HashMap<i32,HashSet<char>> = HashMap::new();
        let mut split = line.split(" | ");
        let input_section = split.next().unwrap();
        let output_section = split.next().unwrap();
        let mut unconsidered = input_section.split(' ').collect::<VecDeque<_>>();
        while number_mappings.len() < 10 {
            let word = unconsidered.pop_front().unwrap();
            let word_set = word.chars().collect::<HashSet<_>>();
            match word_set.len() {
                2 => {number_mappings.insert(1, word_set);},
                3 => {number_mappings.insert(7, word_set);},
                4 => {number_mappings.insert(4, word_set);},
                7 => {number_mappings.insert(8, word_set);},
                5 => {
                    if number_mappings.contains_key(&1) && word_set.is_superset(number_mappings.get(&1).unwrap()) { 
                        number_mappings.insert(3, word_set);
                    } else if number_mappings.contains_key(&9) && word_set.is_subset(number_mappings.get(&9).unwrap()) {
                        number_mappings.insert(5, word_set);
                    } else if number_mappings.contains_key(&9) {
                        number_mappings.insert(2, word_set);
                    } else {
                        unconsidered.push_back(word)
                    }
                },
                6 => {
                    if number_mappings.contains_key(&1) && !word_set.is_superset(number_mappings.get(&1).unwrap()) {
                        number_mappings.insert(6, word.chars().collect::<HashSet<_>>());
                    // Only consider the difference between 9 and 0 once 6 is identified.
                    // 6 is not a superset of 4 or 1, so can't differentiate below until we've
                    // identified it already.
                    } else if number_mappings.contains_key(&6) && number_mappings.contains_key(&4) && word_set.is_superset(number_mappings.get(&4).unwrap()) {
                        number_mappings.insert(9, word.chars().collect::<HashSet<_>>());
                    } else if number_mappings.contains_key(&6) && number_mappings.contains_key(&4) {
                        number_mappings.insert(0, word.chars().collect::<HashSet<_>>());
                    } else {
                        unconsidered.push_back(word)
                    }
                },
                _ => {}
            }
        }
        let mut output = 0;
        for word in output_section.split(' ') {
            if word.len() >= 2 && word.len() <= 4 || word.len() == 7  {
                counter +=1;
            } 
            let char_set = word.chars().collect::<HashSet<_>>();
            for num in number_mappings.keys() {
                if char_set == *number_mappings.get(num).unwrap() {
                    output = (output*10) + num;
                    break;
                }
            }
        }
        total += output;
    }
    println!("Part 1: {}", counter);
    println!("Part 2: {}", total);
}

