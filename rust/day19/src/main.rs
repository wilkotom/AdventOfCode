use std::collections::HashMap;
use regex::Regex;


#[derive(Debug)]
enum RuleReference {
    OtherRules(Vec<Vec<i32>>),
    Letter(char)
}

fn main() {
    let (expressions, messages) = parse_file("./input.txt".to_string());
    let mut rules_cache: HashMap<i32,String> = HashMap::new();
    let rule = "^".to_string() + &rule_to_regex(0, &expressions, &mut rules_cache) + "$";
    println!("Matching Messages: {}", check_messages(rule, messages));


}

fn check_messages(rule: String, messages: Vec<String>) -> i32{

    let re = Regex::new(&rule).unwrap();
    let mut count = 0;
    for message in messages.iter() {
        if re.is_match(message) {
            count +=1;
        }
    }
    count
}

fn parse_file(filename: String) -> (HashMap<i32,RuleReference>, Vec<String>) {
    let mut rules = HashMap::new();
    let file = std::fs::read_to_string(filename).unwrap_or("".to_string());
    let entries: Vec<_> = file.split("\n\n").collect::<Vec<_>>();
    for line in entries[0].split("\n") {
        let tokens: Vec<_> = line.split(": ").collect();
        let rule_number: i32 = tokens[0].parse::<i32>().unwrap();
        let rule_value: RuleReference;
        if tokens[1].contains("\"") {
            rule_value = RuleReference::Letter(tokens[1].chars().nth(1).unwrap());
        } else {
            let mut subrules:Vec<Vec<i32>> = Vec::new();
            for subrule in tokens[1].split(" | ") {
                subrules.push(subrule.split(" ").map(|x| x.parse::<i32>().unwrap()).collect());
            }
            rule_value = RuleReference::OtherRules(subrules);
        }
        rules.insert(rule_number, rule_value);
    }

    (rules, entries.get(1).unwrap_or(&"").split("\n").map(|x| x.to_string()).collect())
}

fn rule_to_regex(rule: i32, ruleset: &HashMap<i32,RuleReference>, rules_cache: &mut HashMap<i32,String>) -> String {
    let mut result: String;
    if rules_cache.contains_key(&rule) {
        rules_cache.get(&rule).unwrap().clone()
    } else {
        match ruleset.get(&rule) {
            Some(RuleReference::Letter(x)) => {result = x.to_string()},
            Some(RuleReference::OtherRules(x)) => {
                if rule == 8 {
                    result = rule_to_regex(42, ruleset, rules_cache) + "+";
                } else if rule == 11 {
                    let mut expanded:Vec<String> = Vec::new();
                    for i in 1..5 {
                        expanded.push(format!("{}{{{}}}{}{{{}}}", rule_to_regex(42, ruleset, rules_cache), i, rule_to_regex(31, ruleset, rules_cache), i));
                    }
                    result = "(".to_string() + &expanded.join("|") + ")"
                } else {

                result = x.iter().map(|l| l.iter().map(|&r| rule_to_regex(r, ruleset, rules_cache)).collect::<Vec<String>>().join("")).collect::<Vec<String>>().join("|");
                result = "(".to_string() + &result.to_owned() + ")";
                }
            }
            _ => {result = String::new();}
        }
    rules_cache.insert(rule, result.to_string());
    result
    }

}