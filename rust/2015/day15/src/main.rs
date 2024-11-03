use std::fs::read_to_string;

#[derive(Debug,Clone)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavour: i32,
    texture: i32,
    calories: i32
}

fn main() {

    let data = read_to_string("./input.txt").unwrap();
    let mut ingredients = Vec::new();
    for line in data.split('\n') {
        let tokens = line.split_ascii_whitespace().collect::<Vec<_>>();
        let capacity = tokens[2][..tokens[2].len() -1].parse::<i32>().unwrap();
        let durability = tokens[4][..tokens[4].len() -1].parse::<i32>().unwrap();
        let flavour = tokens[6][..tokens[6].len() -1].parse::<i32>().unwrap();
        let texture = tokens[8][..tokens[8].len() -1].parse::<i32>().unwrap();
        let calories = tokens[10].parse::<i32>().unwrap();
        ingredients.push(Ingredient{capacity,durability,flavour,texture,calories});
    }
    let mut max_score_part1 = 0;
    let mut max_score_part2 = 0;

    for combination in complements(100, ingredients.len() as i32) {
        let capacity_total = 0.max(ingredients.iter().enumerate().map(|(n, i)| i.capacity * combination[n]).sum::<i32>());
        let durability_total = 0.max(ingredients.iter().enumerate().map(|(n, i)| i.durability * combination[n]).sum::<i32>());
        let flavour_total = 0.max(ingredients.iter().enumerate().map(|(n, i)| i.flavour * combination[n]).sum::<i32>());
        let texture_total = 0.max(ingredients.iter().enumerate().map(|(n, i)| i.texture * combination[n]).sum::<i32>());
        let calories_total = 0.max(ingredients.iter().enumerate().map(|(n, i)| i.calories * combination[n]).sum::<i32>());
        max_score_part1 = max_score_part1.max(capacity_total * durability_total * flavour_total * texture_total);
        if calories_total == 500 {
            max_score_part2 = max_score_part2.max(capacity_total * durability_total * flavour_total * texture_total);
        }
    }

    println!("Part 1: {}", max_score_part1);
    println!("Part 2: {}", max_score_part2);


}

fn complements(target: i32, split: i32) -> Vec<Vec<i32>>{
    match split {
        0 => {Vec::new()}
        1 => { vec![vec![target]]}
        _ => {
            let mut res = Vec::new();
            for i in 0..target+1 {
                let mut next = complements(target -i, split -1);
                for list in next.iter_mut() {
                    list.insert(0, i);
                    
                }
                res.append(&mut next)
            }
            res
        }
    }
}