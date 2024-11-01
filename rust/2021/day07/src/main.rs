fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap().split(',').map(|v| v.parse::<isize>().unwrap()).collect::<Vec<_>>();
    let smallest = *input.iter().min().unwrap_or(&isize::MIN);
    let largest = *input.iter().max().unwrap_or(&isize::MAX);
    let mut min_fuel_part1 = isize::MAX;
    let mut min_fuel_part2 = isize::MAX;

    for dest in smallest..largest+1 {
        let mut part1_fuel = 0;
        let mut part2_fuel = 0;
        for sub in &input {
            let dist = isize::abs(sub - dest);
            part1_fuel += dist;
            part2_fuel += dist * (dist +1) / 2;
        }
        if part1_fuel < min_fuel_part1 {
            min_fuel_part1 = part1_fuel
        }
        if part2_fuel < min_fuel_part2 {
            min_fuel_part2 = part2_fuel
        }
    }
    println!("Minimum fuel Part 1: {}", min_fuel_part1);
    println!("Minimum fuel Part 2: {}", min_fuel_part2);
}