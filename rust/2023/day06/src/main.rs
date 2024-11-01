use std::error::Error;
use aochelpers::get_daily_input;

fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(6,2023)?;
    let races = parse_data(&data);
    println!("Part 1: {}",part1(&races));
    println!("Part 2: {}",part2(&races));
    Ok(())
}

fn part1(races: &[(i64,i64)]) -> i64 {
    races.iter().map(|(d,r)| race(*d,*r)).product::<i64>()
}

fn part2(races: &[(i64,i64)]) -> i64 {
    let (duration,record) = races.iter()
        .fold((0,0), |(td,tr), (d,r)| 
            (td * 10_i64.pow(d.ilog10()+1) + d, tr * 10_i64.pow(r.ilog10()+1) + r));
    race(duration, record)
}
fn race(duration: i64, record: i64) -> i64 {
    let mut min_hold_time = 0;
    
    while (duration - min_hold_time) * min_hold_time <= record {
        min_hold_time += 1
    }
    let mut max_hold_time = duration;
    while (duration - max_hold_time) * max_hold_time <= record {
        max_hold_time -= 1
    }
    max_hold_time - min_hold_time +1
}

fn parse_data(data: &str) -> Vec<(i64,i64)>{
    let mut lines = data.split('\n');
    let mut races = lines.next().unwrap().split(' ').filter_map(|x| x.parse::<i64>().ok()).map(|x| (x,0)).collect::<Vec<_>>();
    let _ = lines.next().unwrap().split(' ').filter_map(|x| x.parse::<i64>().ok()).enumerate().map(|(i, y)| races[i].1 = y).collect::<Vec<_>>();
    races
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = "Time:      7  15   30\nDistance:  9  40  200";

    #[test]
    fn test_parse() {
        assert_eq!(parse_data(DATA), vec![(7,9),(15,40),(30,200)])
    }

    #[test]
    fn test_race() {
        assert_eq!(race(7,9), 4);
        assert_eq!(race(15,40), 8);
        assert_eq!(race(30,200), 9);

    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&[(7,9),(15,40),(30,200)]), 288)
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&[(7,9),(15,40),(30,200)]), 71503)
    }
}
    
