use std::collections::HashMap;

fn main() {
    let target = 33100000;
    let mut houses = HashMap::new();
    for i in 1..1_000_000  {
        for j in (i..1_000_000 ).step_by(i) {
            houses.entry(j).or_insert(0);
            let new_val = houses.get(&j).unwrap() + (i*10);
            houses.insert(j, new_val);
        }
    }
    for i in 1..1_000_000  {
        if houses[&i] >= target{
            println!("Part 1: {}", i);
            break;
        }
    }
    houses = HashMap::new();
    for i in 1..1_000_000  {
        for j in (i..1_000_000 ).step_by(i).take(50) {
            houses.entry(j).or_insert(0);
            let new_val = houses.get(&j).unwrap() + (i*11);
            houses.insert(j, new_val);
        }
    }
    for i in 1..1_000_000  {
        if houses[&i] >= target{
            println!("Part 2: {}", i);
            break;
        }
    }


}

