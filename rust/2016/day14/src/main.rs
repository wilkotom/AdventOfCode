use std::collections::HashMap;


const SALT: &str = "jlmsuwbz";

fn main() {
    let mut counter = 0;
    let mut triples: Vec<i32> = Vec::new();
    let mut calculated: HashMap<i32,String> = HashMap::new();
    println!("Stretch hash of 0: {}", get_or_compute_hash(0, &mut calculated));
    while triples.len() < 64 {
        // let hash_string = format!("{:x}", md5::compute(format!("{}{}", SALT, counter)));
        let hash_string = get_or_compute_hash(counter, &mut calculated);
        for i in 0..hash_string.len() -2 {
            if hash_string.chars().nth(i) == hash_string.chars().nth(i+1) && hash_string.chars().nth(i+1)== hash_string.chars().nth(i+2) {
                let sought = hash_string.chars().nth(i).unwrap().to_string().repeat(5);
                for j in 1..1001 {
                    if get_or_compute_hash(counter + j, &mut calculated).contains(&sought) {
                        triples.push(counter);
                        // println!("{}", counter);
                        println!("{} {}\n{}\n{}", counter, counter + j, get_or_compute_hash(counter, &mut calculated), get_or_compute_hash(counter+j , &mut calculated));
                        break;
                    }

                }
                break;
            }
        }

        counter += 1;

    }
    println!("{:?}", triples);
    println!("{:?}", triples[63]);
}

fn get_or_compute_hash(counter: i32, calculated: &mut HashMap<i32,String>,) -> String{
    match calculated.get(&counter) {
        None => {
            let mut hash = format!("{:x}", md5::compute(format!("{}{}", SALT, counter)));
            for _ in 0..2016 {
                hash = format!("{:x}", md5::compute(hash));
            }
            // let hash = format!("{:x}", md5::compute(format!("{}{}", SALT, counter)));
            calculated.insert(counter, hash.clone());
            hash
        },
        Some(hash) => hash.clone()
    }
}