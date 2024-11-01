use std::collections::HashMap;

fn main() {
    let data = std::fs::read_to_string("./input.txt").unwrap();
    let mut sections = data.split("\n\n");
    let polymer = sections.next().unwrap().split_ascii_whitespace().collect::<String>();
    let mut mappings: HashMap<String, char> = HashMap::new();
    for mapping in sections.next().unwrap().split('\n') {
        let mut words = mapping.split(" -> ");
        mappings.insert(words.next().unwrap().to_string(), words.next().unwrap().chars().nth(0).unwrap());
    }
    let original_poly = polymer.clone();
    println!("Part 1 {}", part2(polymer, &mappings, 10));
    println!("Part 2 {}", part2(original_poly, &mappings, 40));
    
}

fn part2(polymer: String, mappings: &HashMap<String, char>, iterations: i32) -> i64 {
    let mut pair_count: HashMap<String, i64> = HashMap::new();
    for i in 0..polymer.len() -1 { 
        let pair = (&polymer[i..i+2]).to_string();
        pair_count.entry(pair.clone()).or_insert(0);
        let pc = pair_count.get(&pair).unwrap() +1;
        pair_count.insert(pair, pc);
	}
    for _ in 0 ..iterations {
        let mut new_pair_count: HashMap<String, i64> = HashMap::new();
        for pair in pair_count.keys(){
            let inserted = mappings.get(pair).unwrap();
            let first = format!("{}{}", &pair[0..1], inserted);
            let last = format!("{}{}",inserted, &pair[1..]);
            let fc = *new_pair_count.get(&first).unwrap_or(&0);
            let lc = *new_pair_count.get(&last).unwrap_or(&0);
            new_pair_count.insert(first, fc + pair_count.get(pair).unwrap());
            new_pair_count.insert(last, lc + pair_count.get(pair).unwrap());
        }
        pair_count = new_pair_count;
    }
    let counts = char_counts_part_2(&pair_count, polymer.chars().next().unwrap(), polymer.chars().last().unwrap());
    let mut counts = counts.values().collect::<Vec<_>>();
    counts.sort();
    counts[counts.len() -1] - counts[0]

}

fn char_counts_part_2(pair_count: &HashMap<String, i64>, first: char, last: char ) -> HashMap<char,i64> {
    // because pairs pverlap, we count every character but the first and last twice. Bump the counts for those
    let mut char_count: HashMap<char,i64> = HashMap::new();
    char_count.insert(first, 1);
    char_count.insert(last,1);
    for pair in pair_count.keys() {
        for c in pair.chars() {
            char_count.entry(c).or_insert(0);
            let total =  pair_count.get(pair).unwrap() + char_count.get(&c).unwrap();
            char_count.insert(c, total);
        }
    }
    let chars = char_count.keys().cloned().collect::<Vec<_>>();
    // ... then halve all counts at the end
    for c in chars {
        let i = *char_count.get(&c).unwrap();
        char_count.insert(c, i /2);
    }
    char_count
}