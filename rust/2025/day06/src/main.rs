use aochelpers::get_daily_input;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(6,2025)?;
    println!("Part1: {}", part1(&data));
    println!("Part1: {}", part2(&data));
    Ok(())
}

fn part1(data: &str) -> u64 {
    let mut columns = Vec::new();
    let mut answer = 0;
    for line in data.lines() {
        for (i, column) in line.split_ascii_whitespace().enumerate() {
            if columns.len() <=i {
                columns.push(Vec::new());
            }
            if let Ok(val) = column.parse::<u64>() {
                columns[i].push(val);
            } else if column == "+" {
                answer += columns[i].iter().fold(0, |a,v| a+v);
            } else {
                answer += columns[i].iter().fold(1, |a,v| a*v);
            }
        }
    }
    answer
}

fn part2(data: &str) -> u64 {
    let mut column_numbers = Vec::new();
    let mut answer =0;
    for line in data.lines() {
        for (i, c) in line.chars().enumerate() {
            if column_numbers.len() == i {
                column_numbers.push(0);
            }
            if let Some(d) = c.to_digit(10) {
                column_numbers[i] *= 10;
                column_numbers[i] += d as u64;
            } else if c == '+' {
                answer += &column_numbers[i..].iter().take_while(|&&v| v != 0).sum();
            } else if c == '*' {
                answer += &column_numbers[i..].iter().take_while(|&&v| v != 0).product();
            }
        }
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTDATA: &str = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_p1() {
        assert_eq!(part1(TESTDATA), 4277556);
    }


    #[test]
    fn test_p2() {
        assert_eq!(part2(TESTDATA), 3263827);
    }
}