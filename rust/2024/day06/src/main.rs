use std::{collections::HashSet, error::Error};
use aochelpers::{get_daily_input, Coordinate, Direction};


fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(6,2024)?;

    let (arena, guard) = parse_data(&data);
    let (part1, part2) = solve(&arena, guard);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn solve(arena: &HashSet<Coordinate<i32>>, mut guard: Coordinate<i32>) -> (usize, usize) {
    let max_x = arena.iter().map(|c|c.x).max().unwrap();
    let max_y = arena.iter().map(|c|c.y).max().unwrap();
    let mut current_facing = Direction::North;
    let mut visited = HashSet::new();
    let mut possible_blocks = HashSet::new();
    let starting_point = guard;
    while (0..=max_x).contains(&guard.x) && (0..=max_y).contains(&guard.y) {
        visited.insert(guard);
        while arena.contains(&guard.neighbour(current_facing)) {
            current_facing = match current_facing {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
                _ => unimplemented!()
            }
        }
        let next_guard_location = guard.neighbour(current_facing);
        let mut parallel_universe = arena.clone();
        parallel_universe.insert(next_guard_location);
        if !possible_blocks.contains(&next_guard_location) 
                && next_guard_location != starting_point 
                && is_loop(&parallel_universe, max_x, max_y, starting_point) 
                && (0..=max_x).contains(&next_guard_location.x) && (0..=max_y).contains(&next_guard_location.y) {
            possible_blocks.insert(next_guard_location);
        }
        guard = guard.neighbour(current_facing);
    }
    (visited.len(),possible_blocks.len())
}


fn is_loop(arena: &HashSet<Coordinate<i32>>, max_x: i32, max_y: i32, mut guard: Coordinate<i32>) -> bool {
    let mut visited: HashSet<(Coordinate<i32>, Direction)> = HashSet::new();
    let mut current_facing: Direction = Direction::North;
    while (0..=max_x).contains(&guard.x) && (0..=max_y).contains(&guard.y){
        if visited.contains(&(guard,current_facing)) {
            return true;
        }
        visited.insert((guard, current_facing));
        while arena.contains(&guard.neighbour(current_facing)) {
            current_facing = match current_facing {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
                _ => unimplemented!()
            }
        }
        guard = guard.neighbour(current_facing);
    }
    false
}


fn parse_data(input: &str) -> (HashSet<Coordinate<i32>>, Coordinate<i32>) {
    let mut arena = HashSet::new();
    let mut guard_location = Coordinate{x: i32::MAX, y: i32::MAX};
    for (y,line) in input.lines().enumerate() {
        for(x,c) in line.chars().enumerate() {
            match c {
                '#' => {arena.insert(Coordinate{x: x as  i32,y : y as i32});}
                '^' => {guard_location = Coordinate{x: x as i32,y: y as i32};}
                _ => {}
            }
        }
    }


    (arena, guard_location)
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTDATA: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part1() {
        let (arena, guard) = parse_data(TESTDATA);
        assert_eq!(solve(&arena, guard).0,41);

    }

    #[test]
    fn test_part2() {
        let (arena, guard) = parse_data(TESTDATA);
        assert_eq!(solve(&arena, guard).1,6);

    }
}
