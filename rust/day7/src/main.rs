use std::fs;
use std::io::Error;
use std::collections::HashMap;


fn main() -> Result<(), Error> {
    let mappings = parse_bag_file("../input.txt".to_string())?;
    let mut total = 0;
    for outer_bag in mappings.keys().map(|x| x.clone()){
        if can_contain(&outer_bag, &("shiny gold bag".to_string()), &mappings, &mut HashMap::new()) {
            total += 1;
        }
    }
    println!("Part 1 answer: {}", total);
    println!("Part 2 answer: {}", contains(&("shiny gold bag".to_string()), &mappings, &mut HashMap::new()));
    Ok(())
}

fn can_contain(outer_bag: &String, sought: &String, mappings: &HashMap<String,HashMap<String,i32>>, cache: &mut HashMap<String, bool>) -> bool {
    match cache.get(outer_bag).map(|x| x.clone()) {
        Some(result) => result,
        None =>  {  
            if mappings.contains_key(outer_bag) {
                let bag_contents = &mappings[outer_bag];
                if bag_contents.contains_key(sought) {
                    cache.insert(outer_bag.clone(), true);
                    true
                } else {
                let mut contains = false;
                for next_bag in bag_contents.keys().map(|x| x.clone()){
                        let found:bool = can_contain(&next_bag, sought, mappings, cache);
                        cache.insert(next_bag, found);
                        contains = found || contains;
                }
                contains                
                 }
            } else {
                false
            }
        }
    }
}
        
fn contains(outer_bag: &String, mappings: &HashMap<String,HashMap<String,i32>>, cache: &mut HashMap<String, i32>) -> i32 {
    match cache.get(outer_bag).map(|x| x.clone()) {
        Some(result) => result,
        None => {
            if mappings[outer_bag].keys().len() == 0{
                cache.insert(outer_bag.clone(),0);
                0
            } else {
                let mut total = 0;
                for inner_bag in mappings[outer_bag].keys() {
                    total += mappings[outer_bag][inner_bag] * (1+ contains(inner_bag, mappings, cache));
                }
                cache.insert(outer_bag.clone(),total);
                total
            }
        }
    }
}

fn parse_bag_file(filename: String) ->  Result<HashMap<String,HashMap<String,i32>>, Error> {
    let mut mappings: HashMap<String, HashMap<String,i32>> = HashMap::new();
    let raw_data = fs::read_to_string(filename)?;
    let lines: Vec<_> = raw_data.split("\n").collect();
    for line in lines.iter() {
        let line = line.replace("bags", "bag").replace(".", "");
        let details: Vec<_> = line.split(" contain ").collect();
        let outer_bag_name = details[0].to_string();
        let mut inner_bag_details: HashMap<String,i32> = HashMap::new();
        if details[1] != "no other bag" {
            let inner_bags: Vec<_> = details[1].split(", ").collect();
            for bag in inner_bags.iter() {
                let words: Vec<_> = bag.split(" ").collect();
                let quantity: i32 = words[0].to_string().parse().unwrap();
                let inner_bag_name = words[1..].join(" ").to_string();
                inner_bag_details.insert(inner_bag_name, quantity);
            }
            
        }
        mappings.insert(outer_bag_name, inner_bag_details);
        
    }
    Ok(mappings)
}