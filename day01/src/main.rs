use std::fs::read_to_string;

fn main() {
    let data = read_to_string("./day01/input.txt").unwrap().split_ascii_whitespace().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();

    println!("Part 1: {}", data.iter().map(|x| calc_fuel(*x)).sum::<i64>());
    println!("Part 2: {}", data.iter().map(|x| calc_fuel_part_2(*x)).sum::<i64>());

}

fn calc_fuel(object:i64) -> i64 {
    (object /3) -2
}

fn calc_fuel_part_2(object: i64) -> i64 {
    let mut total_fuel = 0;
    let mut additional_fuel = 0.max(calc_fuel(object));
    while additional_fuel > 0 {
        total_fuel += additional_fuel;
        additional_fuel = 0.max(calc_fuel(additional_fuel));
    }
    total_fuel

}