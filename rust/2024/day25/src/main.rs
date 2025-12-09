use std::error::Error;

use aochelpers::get_daily_input;

fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(25,2024)?;
    let (keys, locks) = parse_data(&data);
    println!("Part 1: {}", part1(keys, locks));
    Ok(())
}

fn part1(keys: Vec<[u8;5]>, locks: Vec<[u8;5]>) -> usize {
    let mut valid_combos = 0;
    for key in keys {
        for lock in locks.iter() {
            if key.iter().enumerate().all(|(i,v)| v + lock[i] <=7) {
                valid_combos += 1;
            }
        }
    }
    valid_combos
}

fn parse_data(data: &str) -> (Vec<[u8;5]>, Vec<[u8;5]>) {
    let mut keys = Vec::new();
    let mut locks = Vec::new();
    for section in data.split("\n\n") {
        let mut counts = [0;5];
        for line in section.lines() {
            for (i,c) in line.chars().enumerate() {
                if c == '#' {
                    counts[i] +=1;
                }
            }
        }
        if section.starts_with(".....") {
            keys.push(counts);
        } else {
            locks.push(counts);
        }
    }
    (keys, locks)
}


#[cfg(test)]
mod tests {

    use super::*;

    
    const TESTDATA: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

    #[test]
    fn test_part1() {
        let (keys, locks) = parse_data(TESTDATA);
        assert_eq!(locks, Vec::from([[1,6,4,5,4],[2,3,1,6,4]]));
        assert_eq!(keys, Vec::from([[6,1,3,2,4],[5,4,5,1,3],[4,1,3,1,2]]));
        assert_eq!(part1(keys,locks),3);
    }

    #[test]
    fn test_part2() {
    }
}