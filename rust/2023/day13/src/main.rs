use std::{error::Error, collections::{HashSet, HashMap}};
use aochelpers::get_daily_input;

fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(13,2023)?;
    println!("{}", data.split("\n\n").map(|x| score_mirror(x, find_reflection_line)).sum::<usize>());
    println!("{}", data.split("\n\n").map(|x| score_mirror(x, find_smudge_line)).sum::<usize>());
    Ok(())
}

fn score_mirror(grid: &str, find_fn: fn(&[String]) -> Option<usize> ) -> usize {
    let lines = grid.split('\n').map(|l| l.to_string()).collect::<Vec<_>>();
    let vertical_reflection = find_fn(&lines).unwrap_or(0);
    let horizontal_reflection = find_fn(&reflect_grid(&lines)).unwrap_or(0);
    vertical_reflection + 100* horizontal_reflection
}

fn find_reflection_line(grid: &[String]) -> Option<usize> {
    let mut symmetries  = line_reflection_points(&grid[0]);
    for line in grid {
        let new_symmetries = line_reflection_points(line);
        symmetries = symmetries.intersection(&new_symmetries).copied().collect::<HashSet<_>>();
    }
    symmetries.iter().next().copied()
}

fn find_smudge_line(grid: &[String]) -> Option<usize> {
    let mut symmetry_counts = HashMap::new();
    for new_symmetries in grid.iter().map(|line| line_reflection_points(line)) {
        for symmetry in new_symmetries {
            *symmetry_counts.entry(symmetry).or_insert(0) += 1;
        }
    }
    symmetry_counts.iter().find_map(|(k,v)| if *v == grid.len() -1 {Some(*k)} else {None})
}


fn line_reflection_points(line: &str) -> HashSet<usize> {
    let mut result = HashSet::new();
    for i in 1..line.len() {
        let left = line[0..i].chars().rev().collect::<String>();
        let right = &line[i..];
        let slice_size = left.len().min(right.len());
        if left[..slice_size] == right[..slice_size] {
            result.insert(i);
        }
        if left[..slice_size-1] == right[..slice_size] {
            result.insert(i-1);
        }
    }
    result
}

fn reflect_grid (grid: &[String]) -> Vec<String> {
    let mut new_grid: Vec<_> = Vec::new();
    for col in 0..grid[0].len() {
        let mut new_line = "".to_string();
        for row in grid {
            new_line.push(row.chars().nth(col).unwrap());
        }
        new_grid.push(new_line);
    }
    new_grid
}

#[cfg(test)]
mod tests {
    use super::*; 

    const EXAMPLE1 : &str = 
"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";

const EXAMPLE2 : &str = 
"#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_reflection_point_str() {
        assert_eq!(score_mirror(EXAMPLE1, find_reflection_line), 5);
        assert_eq!(score_mirror(EXAMPLE2, find_reflection_line), 400);
    }

    #[test]
    fn test_smudge_point_str() {
        assert_eq!(score_mirror(EXAMPLE1, find_smudge_line), 300);
        assert_eq!(score_mirror(EXAMPLE2, find_smudge_line), 100);
    }
}