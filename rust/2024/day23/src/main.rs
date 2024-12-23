use std::{collections::{HashMap, HashSet, VecDeque}, error::Error};
use aochelpers::get_daily_input;

fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(23,2024)?;
    let conns = parse_data(&data);
    println!("Part 1: {}", part1(&conns));
    println!("Part 2: {}", part2(&conns));

    Ok(())
}

fn part1<'a>( connections: &HashMap<&'a str, Vec<&'a str>>) -> usize {
    let mut seen: HashSet<&&str> = HashSet::new();
    let mut result = 0;
    for starting_point in connections.keys() {
        if ! starting_point.starts_with("t") || seen.contains(starting_point){
            continue;
        }
        let reachable = connections.get(starting_point).unwrap();
        for (i,left) in reachable.iter().enumerate() {
            if seen.contains(left) {
                continue;
            }
            for right in reachable[i+1..].iter() {
                if connections.get(right).unwrap().contains(left) && !seen.contains(right){
                    seen.insert(starting_point);
                    result +=1;
                }
            }
        } 
    }
    result
}

fn part2<'a>( connections: &HashMap<&'a str, Vec<&'a str>>) -> String {
    let mut biggest = Vec::new();
    for starting_point in connections.keys() {
        let mut group = vec![starting_point];
        for neighbour in connections.get(starting_point).unwrap() {
            if group.iter().all(|node| connections.get(neighbour).unwrap().contains(node)) {
                group.push(neighbour);
            }
        }
        if group.len() > biggest.len() {
            biggest = group
        }

    }
    biggest.sort();
    biggest.into_iter().map(|c|c.to_string()).collect::<Vec<_>>().join(",")
}

fn parse_data<'a>(data:&'a str) -> HashMap<&'a str, Vec<&'a str>> {
    let mut network_map: HashMap<&'a str, Vec<&'a str>> = HashMap::new();
    for line in data.lines() {
        let mut tokens = line.split('-');
        let left = tokens.next().unwrap();
        let right = tokens.next().unwrap();
        network_map.entry(left).or_default().push(right);
        network_map.entry(right).or_default().push(left);
    }
    network_map
}



#[cfg(test)]
mod tests {
    use super::*;
    const TESTDATA: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";
    #[test]
    fn test_part1() {
        let data = parse_data(TESTDATA);
        assert_eq!(part1(&data), 7);
    }

    #[test]
    fn test_part2() {
        let data = parse_data(TESTDATA);
        assert_eq!(part2(&data), "co,de,ka,ta".to_string());
    }
}