use std::collections::HashMap;
use aochelpers::get_daily_input;
use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(7, 2025)?;
    let (p1,p2) = solve(&data);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    Ok(())
}

fn solve(data: &str) -> (usize, usize) {
    let mut p1answer = 0;
    let mut tachyons = HashMap::new();
    for line in data.lines() {
        for (x, c) in line.chars().enumerate() {
            match c {
                'S' => { tachyons.insert(x, 1); }
                '^' => {
                    if let Some(count) = tachyons.remove(&x) {
                        p1answer += 1;
                        *tachyons.entry(x-1).or_default() += count;
                        *tachyons.entry(x+1).or_default() += count;
                    }
                }
                _ => {}
            }
        }
    }
    (p1answer, tachyons.values().sum())
}


#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_solve() {
        let (p1, p2) = solve(TESTDATA);
        assert_eq!(p1, 21);
        assert_eq!(p2, 40);
    }
    
}