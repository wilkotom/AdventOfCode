use std::{error::Error, collections::HashMap};
use aochelpers::{get_daily_input, Label};

#[derive(Debug,Clone,Copy,Eq,PartialEq)]
struct MachinePart {
    x: i128,
    m: i128,
    a: i128,
    s: i128
}
#[derive(Debug,Clone,Copy,Eq,PartialEq)]
struct MachinePartRange {
    lower: MachinePart,
    upper: MachinePart
}

#[derive(Debug,Clone,Copy,PartialEq, Eq)]
enum Threshold {
    Greater,
    Lesser
}
#[derive(Debug,Clone,PartialEq, Eq)]
struct ComparisonRule {
    check_field: char,
    comparator: Threshold,
    value: i128,
    destination: Label
}

fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(19,2023)?;
    let mut fields = data.split("\n\n");
    let ruleset = build_ruleset(fields.next().unwrap());
    let parts = fields.next().unwrap();
    println!("Part 1: {}", part1(&ruleset, parts));
    println!("Part 2: {}", part2(&ruleset));
    Ok(())
}

fn part1(ruleset: &HashMap<Label,Vec<ComparisonRule>>, items: &str) -> i128 {
    items.lines().map(parse_machine_part).filter_map(|i| process_item(&i, ruleset)).sum::<i128>()
}

fn build_ruleset(data: &str) -> HashMap<Label,Vec<ComparisonRule>> {
    let mut rules = HashMap::new();
    for line in data.lines() {
        let splitter = line.find('{').unwrap();
        let label = line[..splitter].parse::<Label>().unwrap();
        rules.insert(label,line[splitter+1..line.len()-1].split(',').map(parse_rule).collect::<Vec<_>>() );
    }
    rules
}

fn process_item(item: &MachinePart, ruleset: &HashMap<Label,Vec<ComparisonRule>>) -> Option<i128> {
    let mut current_stage = "in".parse().unwrap();
    while let Some(next) = get_result_part1(ruleset.get(&current_stage).unwrap(), item) {
        if next == "A".parse().unwrap() {
            return Some(item.x + item.m +item.a + item.s);
        } else if next == "R".parse().unwrap() {
            return None;
        }
        current_stage = next;
    }
    None
}
fn parse_machine_part(line: &str) -> MachinePart {
    let mut values = line[1..line.len()-1].split(',').map(|e| e[2..].parse::<i128>().unwrap());
    MachinePart{ 
        x: values.next().unwrap(),
        m: values.next().unwrap(),
        a: values.next().unwrap(),
        s: values.next().unwrap()
    }
}

fn parse_rule(rule_test: &str) -> ComparisonRule {
    let mut sections = rule_test.split(':');
    if !rule_test.contains(':') {
        return ComparisonRule{
            check_field: 'x',
            comparator: Threshold::Greater,
            value: -1,
            destination: rule_test.parse::<Label>().unwrap()
        };
    }
    let rule = sections.next().unwrap();
    match (rule.chars().next().unwrap(), rule.chars().nth(1).unwrap()) {
        (c, o ) if "xmas".contains(c) && "<>".contains(o) => ComparisonRule{
            check_field: c,
            comparator: if o == '<' {Threshold::Lesser} else {Threshold::Greater},
            value: rule[2..].parse::<i128>().unwrap(),
            destination: sections.next().unwrap().parse::<Label>().unwrap()
         },
        (_,_) => unimplemented!()
    }
}

fn part2(ruleset: &HashMap<Label,Vec<ComparisonRule>>) -> i128 {
    let mut totals = 0;
    let mut unprocessed = Vec::new();
    let starting_range = MachinePartRange{
        lower: MachinePart{ x: 1, m: 1, a: 1, s: 1},
        upper: MachinePart{ x: 4000, m: 4000, a: 4000, s: 4000},
    };
    unprocessed.push(("in".parse::<Label>().unwrap() , starting_range));
    while let Some((rule, range)) = unprocessed.pop() {
        if rule == "A".parse::<Label>().unwrap() {
            totals += (range.upper.x - range.lower.x + 1) *
                      (range.upper.m - range.lower.m + 1) *
                      (range.upper.a - range.lower.a + 1) *
                      (range.upper.s - range.lower.s + 1)
        } else if rule == "R".parse::<Label>().unwrap() {
            continue;
        } else {
            get_rule_result_part2(ruleset.get(&rule).unwrap(), range).into_iter().for_each(|r| unprocessed.push(r));
        }
    }
    totals
}

fn rule_matches(rule: &ComparisonRule, item: &MachinePart) -> bool {
    match (rule.check_field, rule.comparator) {
        ('*', _) => true,
        ('x', Threshold::Greater) => item.x > rule.value,
        ('x', Threshold::Lesser) => item.x < rule.value,
        ('m', Threshold::Greater) => item.m > rule.value,
        ('m', Threshold::Lesser) => item.m < rule.value,
        ('a', Threshold::Greater) => item.a > rule.value,
        ('a', Threshold::Lesser) => item.a < rule.value,
        ('s', Threshold::Greater) => item.s > rule.value,
        ('s', Threshold::Lesser) => item.s < rule.value,
        (_,_) => unimplemented!()
    }
}

fn get_result_part1(rules: &[ComparisonRule], part: &MachinePart) -> Option<Label>{
    for rule in rules.iter() {
        if rule_matches(rule, part) {
            return Some(rule.destination);
        }
    }
    None
}

fn get_rule_result_part2(rules: &[ComparisonRule], mut range: MachinePartRange) -> Vec<(Label, MachinePartRange)> {
    let mut results: Vec<(Label,MachinePartRange)> = Vec::new();
    for rule in rules {
        match (rule.check_field, rule.comparator) {
            ('x', Threshold::Greater) => {
                if range.lower.x > rule.value {
                    results.push((rule.destination, range));
                    break;
                } else if range.upper.x > rule.value {
                    let mut upper_split = range;
                    upper_split.lower.x = rule.value +1;
                    results.push((rule.destination, upper_split));
                    range.upper.x = rule.value;
                }
            },
            ('x', Threshold::Lesser) => {
                if range.upper.x < rule.value {
                    results.push((rule.destination, range));
                    break;
                } else if range.lower.x < rule.value {
                    let mut lower_split = range;
                    lower_split.upper.x = rule.value-1;
                    results.push((rule.destination, lower_split));
                    range.lower.x = rule.value;
                }
            },
            ('m', Threshold::Greater) => {
                if range.lower.m > rule.value {
                    results.push((rule.destination, range));
                    break;
                } else if range.upper.m > rule.value {
                    let mut upper_split = range;
                    upper_split.lower.m = rule.value +1;
                    results.push((rule.destination, upper_split));
                    range.upper.m = rule.value;
                }
            },
            ('m', Threshold::Lesser) => {
                if range.upper.m < rule.value {
                    results.push((rule.destination, range));
                    break;
                } else if range.lower.m < rule.value {
                    let mut lower_split = range;
                    lower_split.upper.m = rule.value-1;
                    results.push((rule.destination, lower_split));
                    range.lower.m = rule.value;
                }
            },
            ('a', Threshold::Greater) => {
                if range.lower.a > rule.value {
                    results.push((rule.destination, range));
                    break;
                } else if range.upper.a > rule.value {
                    let mut upper_split = range;
                    upper_split.lower.a = rule.value +1;
                    results.push((rule.destination, upper_split));
                    range.upper.a = rule.value;
                }
            },
            ('a', Threshold::Lesser) => {
                if range.upper.a < rule.value {
                    results.push((rule.destination, range));
                    break;
                } else if range.lower.a < rule.value {
                    let mut lower_split = range;
                    lower_split.upper.a = rule.value-1;
                    results.push((rule.destination, lower_split));
                    range.lower.a = rule.value;
                }
            },
            ('s', Threshold::Greater) => {
                if range.lower.s > rule.value {
                    results.push((rule.destination, range));
                    break;
                } else if range.upper.s > rule.value {
                    let mut upper_split = range;
                    upper_split.lower.s = rule.value +1;
                    results.push((rule.destination, upper_split));
                    range.upper.s = rule.value;
                }
            },
            ('s', Threshold::Lesser) => {
                if range.upper.s < rule.value {
                    results.push((rule.destination, range));
                    break;
                } else if  range.lower.s < rule.value {
                    let mut lower_split = range;
                    lower_split.upper.s = rule.value-1;
                    results.push((rule.destination, lower_split));
                    range.lower.s = rule.value;
                }
            },
            (_,_) => unimplemented!()
        }
    }

    results
}

#[cfg(test)]
mod tests {

    use super::*; 
    const RULES:&str = 
"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}";

    const ITEMS: &str = 
"{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn test_parse_machine_part() {
        assert_eq!(parse_machine_part("{x=787,m=2655,a=1222,s=2876}"), MachinePart{x:787,m:2655,a:1222,s:2876})
    }


    #[test]
    fn test_parse_rule() {
        assert_eq!( parse_rule("a<2006:qkq"), ComparisonRule{check_field: 'a', comparator: Threshold::Lesser, value: 2006, destination: "qkq".parse().unwrap()});
        assert_eq!( parse_rule("x>42:fixx"), ComparisonRule{check_field: 'x', comparator: Threshold::Greater, value: 42, destination: "fixx".parse().unwrap()});
        assert_eq!( parse_rule("A"), ComparisonRule{check_field: 'x', comparator: Threshold::Greater, value: -1, destination: "A".parse().unwrap()});
    }

    #[test]
    fn test_part1() {
        let ruleset = build_ruleset(RULES);
        assert_eq!(part1(&ruleset, ITEMS), 19114)
    }

    #[test]
    fn test_part2_first_rule() {
        let rules: HashMap<Label, Vec<ComparisonRule>> =build_ruleset(RULES);
        let range = MachinePartRange{
            lower: MachinePart{ x: 0, m: 0, a: 0, s: 0},
            upper: MachinePart{ x: 4000, m: 4000, a: 4000, s: 4000},
        };
        assert_eq!(get_rule_result_part2(&rules[&"in".parse::<Label>().unwrap()], range), vec![
            ("px".parse::<Label>().unwrap(), MachinePartRange { lower: MachinePart { x: 0, m: 0, a: 0, s: 0 }, upper: MachinePart { x: 4000, m: 4000, a: 4000, s: 1350 } }), 
            ("qqz".parse::<Label>().unwrap(), MachinePartRange { lower: MachinePart { x: 0, m: 0, a: 0, s: 1351 }, upper: MachinePart { x: 4000, m: 4000, a: 4000, s: 4000 } })]);
    }
    #[test]
    fn test_part2() {
        let rules: HashMap<Label, Vec<ComparisonRule>> =build_ruleset(RULES);
        assert_eq!(part2(&rules), 167409079868000);
    }
}