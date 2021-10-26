use std::fs;
use std::process;

fn main() {
    let (departure_time, timetable) = read_file("./input.txt".to_string());
    println!("Part 1: {}", part1(&departure_time, &timetable));
    println!("Part 2: {}", part2(&timetable));

}

fn part1(departure_time: &i128, timetable: &Vec<(i128,i128)>) -> i128 {
    let mut first_bus_time = i128::MAX;
    let mut first_bus_number = 0;
    for (_, bus_no) in timetable {

        let next_bus_time = ((departure_time+ bus_no) / bus_no) * bus_no;

        if next_bus_time < first_bus_time { 
            first_bus_time = next_bus_time;
            first_bus_number = *bus_no;
        }
        
    }

    first_bus_number * (first_bus_time - departure_time)
}

fn part2(timetable: &Vec<(i128,i128)>) -> i128 {
    /* Chinese Remainder Theorem. Solved by sieving. */
    let mut start = timetable[0].1;
    let mut target = start;
    for (offset, interval) in &timetable[1..] {
        while ((target + offset)% interval) != 0 {
             
            target += start;
        }
        start *= interval;
    }
    target
}

fn read_file(filename: String) -> (i128, Vec<(i128, i128)>) {
    let raw_file = fs::read_to_string(filename);
    let mut bus_times: Vec<(i128,i128)> = Vec::new();
    let departure_time;
    match raw_file {
        Ok(lines) => {
            let lines: Vec<_> = lines.split("\n").collect();
            departure_time = lines[0].parse().unwrap_or(0);
            let line = lines[1].to_string();
            let times = line.split(",");
            for (pos, time) in times.enumerate() {
                match &time.parse::<i128>() { 
                    Ok(t) => {bus_times.push((pos as i128, *t));},
                    _ => {}
                }
            }

        }
        _ => {
            process::exit(1);
        }
    }
    (departure_time, bus_times)
}