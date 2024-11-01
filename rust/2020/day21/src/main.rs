use std::collections::HashSet;
use std::collections::HashMap;
use std::fs;


fn main() {
    let file = fs::read_to_string("./input.txt").unwrap_or("".to_string());
    let mut allergen_mappings: HashMap<String, HashSet<String>> = HashMap::new();
    let mut ingredient_count: HashMap::<String, i32> = HashMap::new();
    for line in file.split("\n") {
        let sections =line.split(" (contains ").collect::<Vec<_>>();
        // let ingredients = sections[0].split(" ").map(|x| x.to_string()).collect::<HashSet<_>>();
        let allergens = &sections[1][0..sections[1].len()-1].split(", ").map(|x| x.to_string()).collect::<Vec<_>>();
        let ingredients = &sections[0].split(" ").map(|x| x.to_string()).collect::<Vec<_>>();
        for allergen in allergens {
            match allergen_mappings.get(allergen) {
                Some(old_allergens) => {
                    let new_allergens = sections[0].split(" ").map(|x| x.to_string()).collect::<HashSet<_>>();
                    let new_allergens = old_allergens.intersection(&new_allergens).map(|x| x.clone()).collect::<HashSet<_>>();
                    allergen_mappings.insert(allergen.clone(), new_allergens);
                },
                None => {
                    allergen_mappings.insert(allergen.clone(), sections[0].split(" ").map(|x| x.to_string()).collect::<HashSet<_>>());
                }

            }
        }
        for ingredient in ingredients.iter() {
            ingredient_count.insert(ingredient.clone(), *ingredient_count.get(ingredient).unwrap_or(&0) + 1);  
        }
    }
    let mut allergens = allergen_mappings.keys().cloned().collect::<Vec<_>>();
    allergens.sort();
    while allergen_mappings.values().map(|x| x.len()).max().unwrap() > 1 {
        for allergen in allergens.iter() {
            if allergen_mappings.get(allergen).unwrap_or(&HashSet::new()).len() == 1 {
                let known = allergen_mappings.get(allergen).unwrap().iter().cloned().collect::<Vec<_>>();
                for other in allergens.iter().filter(|x| *x != allergen) {
                    allergen_mappings.get_mut(other).unwrap().remove(&known[0]);
                }
            }
        }
    }
    let allergens = allergens.iter().map(|x| allergen_mappings.get(x).unwrap().iter().next().unwrap()).collect::<Vec<_>>();
    let mut part2 = String::new();
    for allergen in allergens{
        ingredient_count.remove(allergen);
        part2.push_str(allergen);
        part2.push(',');
    }
    part2.pop();
    println!("Safe Ingredient count: {:?}", ingredient_count.values().sum::<i32>());
    println!("Canonical dangerous ingredients: {}", part2);
}
