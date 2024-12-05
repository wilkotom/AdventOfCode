use std::{cmp::Ordering, collections::{HashMap, HashSet}, error::Error};
use aochelpers::get_daily_input;

fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(05,2024)?;
    let (order,updates) = parse_data(&data);
    println!("Answers: {:?}", answer(&order, &updates));

    Ok(())
}

fn answer(orders: &HashMap<i32,HashSet<i32>>, updates: &[Vec<i32>]) -> (i32, i32) {
    let mut part1 = 0;
    let mut part2 = 0;
    for update in updates {
        let mut sorted = update.clone();
        sorted.sort_by(|a,b| if orders.get(a).unwrap_or(&HashSet::new()).contains(b){Ordering::Less} else {Ordering::Greater});
        if update == &sorted {
            part1 += update[(update.len()) /2]
        } else {
            part2 += sorted[(sorted.len()) /2]
        }
    }
    (part1, part2)
}

fn parse_data(data: &str) -> (HashMap<i32,HashSet<i32>>, Vec<Vec<i32>> ){
    let mut orders: HashMap<i32,HashSet<i32>> = HashMap::new();
    let mut updates = Vec::new();
    for line in data.lines() {
        if line.contains("|") {
            let mut sections = line.split("|");
            let left = sections.next().unwrap().parse().unwrap();
            let right= sections.next().unwrap().parse().unwrap();
            orders.entry(left).or_default().insert(right);
        } else if line.contains(",") {
            let update = line.split(",").map(|v|v.parse().unwrap_or(0)).collect();
            updates.push(update);
        }

    }
    (orders, updates)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part1() {
        let (order,updates) = parse_data(TESTDATA);
        assert_eq!(answer(&order, &updates), (143,123));
    }

    #[test]
    fn test_part2() {
    }
}