use std::{fs::read_to_string, collections::{HashMap, HashSet, BinaryHeap}, cmp::Ordering};

#[derive(Clone,Debug, PartialEq, Eq, Hash)]
struct MoleculeState {
    iterations: i32,
    molecule: String
}

impl Ord for MoleculeState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.molecule.len().cmp(&self.molecule.len())
    }

}

impl PartialOrd for MoleculeState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


fn main() {
    let data = read_to_string("./input.txt").unwrap();
    let mut sections = data.split("\n\n");


    let replacements = sections.next().unwrap();
    let mut mappings: HashMap<String,Vec<String>> = HashMap::new();
    let mut reverse_mappings: HashMap<String, String> = HashMap::new();

    for line in replacements.split('\n'){
        let mut mapping = line.split(" => ");
        let ingredient = mapping.next().unwrap();
        let result = mapping.next().unwrap();
        mappings.entry(ingredient.to_owned()).or_default().push(result.to_owned());
        reverse_mappings.insert(result.to_owned(), ingredient.to_owned());
    
    }


    let target = sections.next().unwrap().to_owned();

    println!("Part 1: {}", part1(&target, &mappings).len());

    let mut heap: BinaryHeap<MoleculeState> = BinaryHeap::new();
    let mut seen: HashSet<String> = HashSet::new();
    heap.push(MoleculeState{iterations: 0, molecule: target});

    while !heap.is_empty() {
        let state = heap.pop().unwrap();
        if state.molecule == *"e" {
            println!("Part 2: {}", state.iterations);
            break;
        } else if seen.contains(&state.molecule) {
            continue;
        }
        seen.insert(state.molecule.to_owned());

        for replacement in reverse_mappings.keys() {
            if state.molecule.contains(replacement){
                let next_mol = state.molecule.replacen(replacement, reverse_mappings.get(replacement).unwrap(), 1);
                heap.push(MoleculeState{iterations: state.iterations +1, molecule: next_mol});
            }
        }
    }
}

fn part1(input: &str, mappings: &HashMap<String,Vec<String>>)-> HashSet<String> {
    let molecule = split_molecule(input);
    let mut results: HashSet<String> = HashSet::new();
    for (i, atom) in molecule.iter().enumerate() {
        let first = molecule[..i].iter().map(|m| m.to_owned()).collect::<String>();
        let last = molecule[i+1..].iter().map(|m| m.to_owned()).collect::<String>();
        for replacement in mappings.get(atom).unwrap_or(&vec![atom.to_owned()]) {
            let mut everything = first.clone();
            everything.push_str(replacement);
            everything.push_str(&last);
            results.insert(everything);
        }

    }
    results.remove(&(molecule.iter().map(|a| a.to_owned()).collect::<String>()));
    results
}

fn split_molecule(molecule: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = "".to_owned();
    for c in molecule.chars() {
        if c.is_ascii_uppercase() {
            if !current.is_empty() {
                result.push(current);
            }
            current = "".to_owned();
        }
        current.push(c);
    }
    result.push(current);
    result
}
