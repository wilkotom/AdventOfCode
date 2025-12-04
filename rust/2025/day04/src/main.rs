use aochelpers::{Coordinate, get_daily_input};
use std::{collections::HashSet, error::Error};


fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(4, 2025)?;
    let arena = parse_data(&data);
    println!("Part 1: {}", part1(&arena).len());
    println!("Part 2: {}", part2(arena));
    Ok(())
}

fn parse_data(data: &str) -> HashSet<Coordinate<usize>> {
    let mut arena = HashSet::new();
    for (y, line) in data.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '@' {
                arena.insert(Coordinate { x, y});
            }
        }
    }
    arena
}

fn part1(arena: &HashSet<Coordinate<usize>>) -> Vec<Coordinate<usize>>{
   arena.iter().filter(|c| c.checked_extended_neighbours().filter(|n| arena.contains(&n)).count() <4).map(|&c| c).collect::<Vec<Coordinate<usize>>>()
}

fn part2(mut arena: HashSet<Coordinate<usize>>) -> usize {
    let mut answer = 0;
    let mut to_be_removed = part1(&arena);
    while let Some(roll) = to_be_removed.pop() {
        if arena.remove(&roll) {
            answer +=1;
            roll.checked_extended_neighbours().filter(
                    |n| arena.contains(&n) && n.checked_extended_neighbours().filter(|c| arena.contains(&c)).count() < 4)
                .for_each(|n| to_be_removed.push(n));
        }
    }
    answer
}

#[cfg(test)]
mod tests {
    const TESTDATA: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    use super::*;

    #[test]
    fn test_part1() {
        let arena = parse_data(TESTDATA);
        assert_eq!(part1(&arena).len(),13);
    }


    #[test]
    fn test_part2() {
        let arena = parse_data(TESTDATA);
        assert_eq!(part2(arena),43);
    }
    
}