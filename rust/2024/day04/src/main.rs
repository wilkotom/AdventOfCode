use std::{error::Error, usize};
use aochelpers::{get_daily_input, Coordinate, Direction, Grid};

fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(04,2024)?;
    let grid = parse_data(&data);
    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&grid));

    Ok(())

}

fn part1(grid: &Grid<char>) -> usize {
    let mut score = 0;
    for point in grid.keys::<usize>() {
        for direction in [Direction::North, Direction::NorthEast, Direction::East, Direction::SouthEast] {
            let mut location = point;
            let mut readout = String::new();
            for _ in 0..4 {
                readout.push(grid.get(&location).unwrap_or('#'));
                if let Some(next_location) = location.checked_neighbour(direction) {
                    location = next_location;
                } else {
                    break;
                }
            }
            if readout == "XMAS" || readout == "SAMX" {
                score +=1;
            }
        }
    }
    score
}

fn part2(grid: &Grid<char>) -> usize {

    let mut score = 0;
    for (point, c) in grid.iter::<usize>() {
        if c == 'A' {
            let mut diagonal = String::new();
            diagonal.push(grid.get(&point.checked_neighbour(Direction::NorthWest).unwrap_or(Coordinate{x:usize::MAX, y:usize::MAX})).unwrap_or('#'));
            diagonal.push('A');
            diagonal.push(grid.get(&point.checked_neighbour(Direction::SouthEast).unwrap_or(Coordinate{x:usize::MAX, y:usize::MAX})).unwrap_or('#'));
            if ! ["MAS", "SAM"].contains(&&diagonal[..]) {
                continue;
            }
            diagonal.clear();
            diagonal.push(grid.get(&point.checked_neighbour(Direction::NorthEast).unwrap_or(Coordinate{x:usize::MAX, y:usize::MAX})).unwrap_or('#'));
            diagonal.push('A');
            diagonal.push(grid.get(&point.checked_neighbour(Direction::SouthWest).unwrap_or(Coordinate{x:usize::MAX, y:usize::MAX})).unwrap_or('#'));

            if ["MAS", "SAM"].contains(&&diagonal[..]) {
                score+=1;
            }
        }
    }
    score
}

fn parse_data(data: &str) -> Grid<char>{
    let mut grid = Grid::new();
    for (y,line) in data.lines().enumerate() {
        for (x,c) in line.chars().enumerate(){
            grid.insert(Coordinate{x,y}, c);
        }
    }
    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA:&str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    #[test]
    fn test_part1() {
        let grid = parse_data(TESTDATA);
        assert_eq!(part1(&grid),18);
        
    }

    #[test]
    fn test_part2() {
        let grid = parse_data(TESTDATA);
        assert_eq!(part2(&grid),9);

    }
}