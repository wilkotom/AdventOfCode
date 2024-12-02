use std::error::Error;
use aochelpers::get_daily_input;

fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(02,2024)?;
    let reports = parse_data(&data);
    let (part1, part2) = answer(&reports);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn parse_data(data: &str) -> Vec<Vec<i64>> {
    data.lines().map(|l| l.split_ascii_whitespace().map(|x| x.parse().unwrap()).collect()).collect()
}

fn answer(reports: &[Vec<i64>]) -> (i64, i64) {
    let mut part1 = 0;
    let mut part2 = 0;
    for report in reports {
        if is_safe(report) {
            part1 += 1;
            part2 += 1;
        } else {
            for i in 0..report.len() {
                let mut candidate = report.clone();
                candidate.remove(i);
                if is_safe(&candidate) {
                    part2 += 1;
                    break;
                }
            }
        }
    }
    (part1, part2)
}

fn is_safe(report: &[i64]) -> bool {
    let sign = (report[0] - report[1]).signum();
    report [1..].iter().enumerate().all(|(i,n)| (1..=3).contains(&report[i].abs_diff(*n)) && (report[i] - n).signum() == sign)
}

#[cfg(test)]
mod tests {
    
    const TESTDATA: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    use super::*;

    #[test]
    fn test_parts() {
        let data: Vec<Vec<i64>> = parse_data(TESTDATA);
        assert_eq!(data,vec![vec![7,6,4,2,1], vec![1,2,7,8,9], vec![9,7,6,2,1], vec![1,3,2,4,5], vec![8,6,4,4,1], vec![1,3,6,7,9]]);
        assert_eq!(answer(&data),(2,4));
    }

}