use std::{collections::HashMap, fs};
use std::cmp::Reverse;

fn main() {
    let input = fs::read_to_string(String::from("./input.txt")).unwrap();
    let rooms = get_valid_room(input);
    for room in rooms {
        let result = decrypt_room_name(room);
        if result.0 == *"northpole object storage"{
            println!("Part 2 answer: {:?}", result.1);
        }
        
    }
}

fn get_valid_room(input: String) -> Vec<String> {
    let mut rooms: Vec<String> = Vec::new();
    let mut sector_ids_sum = 0;
    for line in input.split("\n"){
        let mut letter_counts: HashMap<char, i32> = HashMap::new();
        let mut tokens = line.split("[");
        let room_name = tokens.next().unwrap();
        let supplied_checksum = &tokens.next().unwrap()[0..5];

        for c in room_name.chars() {
            if let 'a'..='z' = c {letter_counts.insert(c, *letter_counts.get(&c).unwrap_or(&0) +1);}
        }
        let mut letters_by_count: HashMap<i32, Vec<char>> = HashMap::new();
        for letter in letter_counts.keys() {
            let count = letter_counts.get(letter).unwrap();
            if !letters_by_count.contains_key(count) {
                letters_by_count.insert(*count, vec![*letter]);
            } else {
                letters_by_count.get_mut(count).unwrap().push(*letter);
            }

        }
        let mut counts_in_order = letters_by_count.keys().copied().collect::<Vec<_>>();
        counts_in_order.sort_by_key(|w| Reverse(*w));
        let mut checksum: String = String::new();
        for count in &counts_in_order {
            if checksum.len() <5 {
                let mut letters = letters_by_count.get(count).unwrap().clone();
                letters.sort();
                for letter in letters {
                    checksum.push(letter);
                }
            } else {
                break
            }
        }
        if checksum.len() > 5 {
            checksum.truncate(5);
        }
        if  checksum == supplied_checksum {
            sector_ids_sum += room_name.split("-").last().unwrap().parse::<i32>().unwrap();
            rooms.push(String::from(room_name));
        }

    }
    println!("Part 1 answer: {}", sector_ids_sum);
    rooms
}

fn decrypt_room_name(room_name: String) -> (String, u32) {
    let mut decrypted = String::new();
    let sector_id= room_name.split("-").last().unwrap().parse::<u32>().unwrap();
    for c in room_name.chars() {
        let d = match c {
            'a'..='z' => {
                let offset = sector_id % 26;
                let mut n: u8 = (c as u8) + (offset as u8);
                if n > 122 {
                    n -= 26;
                } 
                Some(n as char)
            },
            '-' => Some(' '),
            _ => None
        };
        if let Some(x) = d { decrypted.push(x) }
    }
    decrypted.pop();


    (decrypted, sector_id)
}