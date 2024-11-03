use std::fs;
use std::io::Error;
use std::collections::HashMap;
use regex::Regex;

fn main() -> Result<(), Error> {
    let data = fs::read_to_string("./input.txt")?;
    let mut part1_count = 0;
    let mut part2_count = 0;
    let validations: HashMap<&str, Regex> = [
        ("byr", Regex::new(r"^(19[2-9][0-9]|200[012])$").unwrap()),
        ("iyr", Regex::new(r"^20([1][0-9]|20)$").unwrap()),
        ("eyr", Regex::new(r"^20(2[0-9]|30)$").unwrap()),
        ("hgt", Regex::new(r"^(1([5-8][0-9]|9[0-3])cm|(59|6[0-9]|7[0-6])in)$").unwrap()),
        ("hcl", Regex::new(r"^#[0-9a-f]{6}$").unwrap()),
        ("ecl", Regex::new(r"^(amb|b(lu|rn)|gr[yn]|hzl|oth)$").unwrap()),
        ("pid", Regex::new(r"^[0-9]{9}$").unwrap())]
        .iter().cloned().collect();
    let records: Vec<_> = data.split("\n\n").collect();
    for record in records.iter() {
        let fields : Vec<_> = record.split_whitespace().collect();
        let mut passport = HashMap::new();
        for field in fields.iter() {
            let detail: Vec<_> = field.split(":").collect();
            passport.insert(detail[0], detail[1]);
        }
        if passport.contains_key("cid"){
            passport.remove("cid");
        }
        let mut valid_1 = true;
        let mut valid_2 = true;
        for (key, re) in &validations {
            match passport.get(key) {
                Some(data) => {
                    if !re.is_match(data) {
                        valid_2 = false;
                    }
                }
                _ => { valid_1 = false;
                    valid_2 = false;
}            }
        }
        if valid_1 {
            part1_count +=1;
        }
        if valid_2 {
            part2_count +=1;
        }

    }
    println!("Part 1 Correct Passports: {}", part1_count);
    println!("Part 2 Correct Passports: {}", part2_count);

    Ok(())
}