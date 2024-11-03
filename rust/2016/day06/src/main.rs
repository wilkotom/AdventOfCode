use std::collections::HashMap;

fn main() {
    let file  = std::fs::read_to_string(String::from("./input.txt")).unwrap();
    let mut char_counts: HashMap<usize, HashMap<char, i32>> = HashMap::new();
    for line in file.split("\n"){
       for (i, c) in line.chars().enumerate() {
            char_counts.entry(i).or_default();
            let counts = char_counts.get_mut(&i).unwrap();
            counts.insert(c, counts.get(&c).unwrap_or(&0) +1);

       }
    }
    let mut easter_message = String::new();
    let mut christmas_message = String::new();

    for pos in 0..char_counts.len() {
        let mut max_count = 0;
        let mut min_count = i32::MAX;

        let mut e = '_';
        let mut c = '_';
        for (k, v) in char_counts.get(&pos).unwrap() {
            if v > &max_count {
                max_count = *v;
                e = *k;
            }
            if v < &min_count {
                min_count = *v;
                c = *k;
            }
        }
        easter_message.push(e);
        christmas_message.push(c)

    }
    println!("{:?}", easter_message);
    println!("{:?}", christmas_message);

}
