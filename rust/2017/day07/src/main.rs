use std::fs::read_to_string;
use hashbrown::HashMap;

#[derive(Debug,Clone)]
struct Program{
    weight: i64,
    holding: Vec<String>,
    held_by: Option<String>
}



fn main() {
    let data = read_to_string("./input.txt").unwrap();
    let mut programs: HashMap<String,Program> = HashMap::new();
    for line in data.split('\n') {
        let mut fields = line.split(" -> ");
        let mut program_data = fields.next().unwrap().split_ascii_whitespace();
        let name = program_data.next().unwrap().to_string();
        let weight_str = program_data.next().unwrap();
        let weight = weight_str[1..weight_str.len()-1].parse::<i64>().unwrap();
        let holding = fields.next().unwrap_or("").split(", ").filter(|x| ! x.is_empty()).map(String::from).collect::<Vec<_>>();
        for held in &holding {
            if programs.contains_key(held) {
                programs.get_mut(held).unwrap().held_by = Some(name.clone());
            } else {
                programs.insert(held.to_owned(), Program{weight: -1, holding: Vec::new(), held_by: Some(name.clone())});
            }
        }

        if programs.contains_key(&name) {
            let program = programs.get_mut(&name).unwrap();
            program.weight = weight;
            program.holding = holding;
        } else {
            programs.insert(name, Program{weight, holding, held_by: None});
        }

    }
    let mut cached_weights: HashMap<String, i64> = HashMap::new();
    let mut unbalanced_node = "";

    for program in programs.keys() {
        if programs[program].held_by == None {
            println!("Part 1: {}", program);
            unbalanced_node = program;
        }
    }

    println!("Part 2: {}", rebalance(unbalanced_node, 0, &mut cached_weights, &programs));
}

fn rebalance(leaf_name: &str, desired_weight: i64, cached_weights: &mut HashMap<String, i64>, trees: &HashMap<String,Program>) -> i64{

    println!("Rebalancing {} to be weight {}", leaf_name, desired_weight);
    let weights = trees[leaf_name].holding.iter().map(|x| get_weight(x.clone(), cached_weights, trees)).collect::<Vec<_>>();

    let common = if weights[0] == weights[1] || weights[0] == weights[2] { weights[0] } else { weights[1] };
    let mut unbalanced = usize::MAX;
 
    for (i, weight) in weights.iter().enumerate() {
        if *weight != common {
            unbalanced = i;
            break;
        }
    }

    if unbalanced == usize::MAX {
        desired_weight - weights.iter().sum::<i64>()
    } else {
        rebalance(&trees[leaf_name].holding[unbalanced], common, cached_weights, trees)
        
    }

}

fn get_weight(tree_name: String, weights: &mut HashMap<String, i64>, trees: &HashMap<String,Program>) -> i64{

    if weights.contains_key(&tree_name) {
        return weights[&tree_name];
    }

    let tree= trees.get(&tree_name).unwrap();
    let mut weight = tree.weight;
    for subtree in &tree.holding {
        weight += get_weight(subtree.clone(), weights, trees);
    }
    weights.insert(tree_name, weight);
    weight
}
