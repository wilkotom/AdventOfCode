use std::{error::Error, collections::HashMap};
use aochelpers::{get_daily_input,lcm, Label};

#[derive(Debug,PartialEq, Eq)]
struct NextStage  {
    right: Label,
    left: Label
}

fn main() -> Result<(), Box<dyn Error>>{
    let (directions, map) = parse_data(&get_daily_input(8,2023)?);
    println!("Part 1: {}", follow_directions(&directions, &map, &"AAA".parse::<Label>().unwrap()));
    println!("Part 2: {}", direct_ghosts(&directions, &map));
    Ok(())
}

fn follow_directions(directions: &str, map: &HashMap<Label, NextStage>, start: &Label) -> usize {
    let mut steps = 0;
    let mut current_location = *start;
    while !current_location.ends_with('Z') {
        if directions.chars().nth(steps % directions.len()).unwrap() == 'R' {
            current_location = map.get(&current_location).unwrap().right;
        } else {
            current_location = map.get(&current_location).unwrap().left;
        }
        steps += 1
    }
    steps
}

fn direct_ghosts(directions: &str, map: &HashMap<Label, NextStage>) -> usize {
    map.keys().filter(|x| x.ends_with('A')).map(|x| follow_directions(directions, map, &x)).reduce(lcm).unwrap()
}

fn parse_data(data: &str) -> (String, HashMap<Label, NextStage>) {
    let mut sections = data.split("\n\n");
    let steps = sections.next().unwrap().parse().unwrap();
    let mut map = HashMap::new();
    for line in sections.next().unwrap().split('\n') {
        map.insert(line[0..3].parse().unwrap(), NextStage{left:line[7..10].parse().unwrap(), right: line[12..15].parse().unwrap()});
    }
    (steps, map)
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_parser() {
        let data =  "RL\n\nAAA = (BBB, CCC)";
        assert_eq!(parse_data(data), 
            ("RL".to_owned(), 
            HashMap::from([("AAA".parse().unwrap(), NextStage{left: "BBB".parse().unwrap(), right: "CCC".parse().unwrap()})])));
    }

    #[test]
    fn test_score_part1() {
        let (directions, map) = parse_data(DATA);
        assert_eq!(follow_directions(&directions, &map, &"AAA".parse().unwrap()), 2);
        let (directions, map)= parse_data("LLR\n\nAAA = (BBB, BBB)\nBBB = (AAA, ZZZ)\nZZZ = (ZZZ, ZZZ)");
        assert_eq!(follow_directions(&directions, &map, &"AAA".parse().unwrap()), 6);
    }

    #[test]
    fn test_part2() {
        let data = "LR\n\n11A = (11B, XXX)\n11B = (XXX, 11Z)\n11Z = (11B, XXX)\n22A = (22B, XXX)\n22B = (22C, 22C)\n22C = (22Z, 22Z)\n22Z = (22B, 22B)\nXXX = (XXX, XXX)";
        let (directions, map) = parse_data(data);
        assert_eq!(direct_ghosts(&directions, &map), 6);

    }
}