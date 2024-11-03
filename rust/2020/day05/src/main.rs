use std::fs;
use std::io::Error;

fn main() -> Result<(), Error> {
    let raw_data = fs::read_to_string("../input.txt")?;
    let split_data: Vec<_> = raw_data.split("\n").collect();
    let mut seat_numbers: Vec<isize> = Vec::new();
    for record in split_data.iter() {
        let raw_binary = record.replace("F", "0").replace("B", "1").replace("L", "0").replace("R","1");
        let intval = isize::from_str_radix(&raw_binary, 2).unwrap();
        seat_numbers.push(intval);
    }
    let min_value = seat_numbers.iter().min().unwrap();
    let max_value = seat_numbers.iter().max().unwrap();
    let all_nums: isize = seat_numbers.iter().sum();
    let all_seats: isize = (*min_value .. *max_value +1).sum();
    println!("Part 1 answer: {}", *max_value);
    println!("Part 2 answer: {}", all_seats - all_nums);
    // println!("MIssing seat number: {}", all_seats - all_nums);
    Ok(())
}
