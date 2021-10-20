use std::fs::File;
use std::io::{BufRead, BufReader, Error};

struct ParsedRule {
    min: i32,
    max: i32,
    letter: char
}

fn main() -> Result<(), Error> {
    let mut sled_valid = 0;
    let mut toboggan_valid = 0;
    let br = BufReader::new(File::open("./input.txt")?);
    for line in br.lines() {
        match line {
            Ok(line) => {
                let (parsed_rule, password)  = parse_rule(&line);
                if toboggan_validator(&parsed_rule, &password) {
                    toboggan_valid +=1;
                }
                if sled_validator(&parsed_rule, &password) {
                    sled_valid +=1;
                }
            }
            Err(_) => {
                println!("OOps")
            }
        }

    }
    println!("Valid Sled Co Passwords: {}", sled_valid);
    println!("Valid Toboggan Co Passwords: {}", toboggan_valid);

    Ok(())
}

fn parse_rule(line: &str) -> (ParsedRule, &str) {
    let fields: Vec<&str> = line.split(": ").collect();
    let rule: Vec<&str> = fields[0].split(" ").collect();
    let minmax: Vec<i32> = rule[0].split("-").map(|n| n.parse::<i32>().unwrap()).collect();
    let letter = rule[1].chars().nth(0).unwrap();
    (ParsedRule{
        min: minmax[0],
        max: minmax[1],
        letter: letter
    }, 
    fields[1])
}

fn sled_validator(rule: &ParsedRule, password: &str) -> bool {
    let mut occurences = 0;
    for c in password.chars() {
        if c == rule.letter {
            occurences += 1;
        }
    }
    rule.min <= occurences && rule.max >= occurences
}

fn toboggan_validator(rule: &ParsedRule, password: &str) -> bool {
    let first = password.chars().nth((rule.min -1) as usize).unwrap();
    let last = password.chars().nth((rule.max -1) as usize).unwrap();
     (first == rule.letter && last != rule.letter) ||
     (first != rule.letter && last == rule.letter)
}