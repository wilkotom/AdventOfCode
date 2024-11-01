use std::{error::Error, collections::{HashMap, HashSet, BinaryHeap}};
use aochelpers::{get_daily_input, ScoredItem, Label};

fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(25,2023)?;
    let mut parsed =  parse_data(&data);
    let mut indirect_routes = BinaryHeap::new();
    for (key, val) in parsed.iter()  {
        for dest in val {
            indirect_routes.push(ScoredItem{cost: 0- indirect_distance(key, dest, &parsed), item: (key.to_owned(), dest.to_owned())})
        }
    }
    for _ in 0..6 {
        let (start, end) = indirect_routes.pop().unwrap().item;
        let destinations = parsed.get_mut(&start).unwrap();
        destinations.retain(|c| *c != end);
    }
    println!("Answer: {}", answer(&parsed));
    Ok(())
}

fn parse_data(data: &str) -> HashMap<Label, Vec<Label>> {
    let mut parsed = HashMap::new();
    for line in data.lines(){
        let mut nodes = line.split(' ');
        let source = nodes.next().unwrap();
        let src_label = source[0..source.len()-1].parse::<Label>().unwrap();
        for target in nodes {
            let target_label = target.parse::<Label>().unwrap();
            parsed.entry(src_label).or_insert_with (Vec::new).push(target_label);
            parsed.entry(target_label).or_insert_with (Vec::new).push(src_label);
        }
    }
    parsed
}

fn indirect_distance(start:&Label, end: &Label, arena: &HashMap<Label, Vec<Label>>) -> i64 {

    let mut unvisited = BinaryHeap::new();
    let mut visited: HashSet<Label> = HashSet::new();
    visited.insert(*start);
    for next in arena.get(start).unwrap().iter().filter(|x| *x != end) {
        unvisited.push(ScoredItem{cost: 1, item: next.to_owned()});
    }
    while let Some(next_node) = unvisited.pop() {
        // println!("{:?}",next_node);
        if visited.contains(&next_node.item) {
            continue;
        }
        visited.insert(next_node.item);
        if next_node.item == *end {
            return next_node.cost as i64
        } 
        for next in arena.get(&next_node.item).unwrap().iter() {
            unvisited.push(ScoredItem{cost: next_node.cost +1, item: next.to_owned()});
        }
    }

    0
}

fn answer(arena: &HashMap<Label, Vec<Label>>) -> usize {
    let start = arena.keys().next().unwrap();
    let mut visited: HashSet<Label> = HashSet::new();
    let mut unvisited = Vec::new();
    unvisited.push(start.to_owned());
    while let Some(node) = unvisited.pop() {
        if visited.contains(&node) {
            continue;
        }
        visited.insert(node);
        for next in arena.get(&node).unwrap().iter() {
            unvisited.push(next.to_owned());
        }

    }

    (arena.len() - visited.len()) * visited.len()
}

#[cfg(test)]
mod tests {

    use super::*; 
    const DATA: &str = 
"jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

#[test]
fn test_parser() {
    println!("{:?}", parse_data(DATA).len());
}
}