use std::{error::Error, collections::{HashSet, VecDeque}};
use aochelpers::{get_daily_input, Coordinate};

fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(21,2023)?;
    let (gardens, start) = parse_data(&data);
    println!("Part 1: {}", part1(&gardens, &start, 64));
    println!("Part 2: {}", part2(&gardens, &start, 26501365));

    Ok(())
}

fn part1(gardens: &HashSet<Coordinate<i32>>, starting_point: &Coordinate<i32>, step_limit: usize) -> usize {
    let mut seen_states = HashSet::new();
    let mut next_steps: VecDeque<(Coordinate<i32>, usize)> = VecDeque::new();
    let mut destinations = 0;
    next_steps.push_back((*starting_point, 0_usize));
    let original_bound: Coordinate<i32> = Coordinate{ 
        x: gardens.iter().map(|c| c.x).max().unwrap(),
        y: gardens.iter().map(|c| c.y).max().unwrap()
    }; 
    while let Some((location, steps_taken)) = next_steps.pop_front() {

        if seen_states.contains(&location) {
            continue;
        }
        seen_states.insert(location);
        if step_limit %2 == steps_taken %2{
            destinations +=1;
        }
        if steps_taken == step_limit {
            continue;
        } 
        for neighbour in location.neighbours() {
            if gardens.contains(&Coordinate{ x: neighbour.x.rem_euclid(original_bound.x +1), y: neighbour.y.rem_euclid(original_bound.y +1) }) {
                next_steps.push_back((neighbour, steps_taken+1));
            }
        }
    } 
    destinations
}

fn part2(gardens: &HashSet<Coordinate<i32>>, starting_point: &Coordinate<i32>, walk_len: usize) -> usize {
    // grid for part 2 is 131 characters wide.
    // We want to walk up to 26501365 squares, which is to say:
    // 65 squares plus 202300 times the length of the grid
    // 65 = number of steps to reach the edge from the starting point
    // This will only solve for the case where 
    // steps mod squaresize == square size/2 -1
    let max_grid_traversals = walk_len / gardens.iter().map(|c| c.x).max().unwrap() as usize;
    let mut progression = Vec::new();
    
    // number of input squares it's possible to visit increases with geometric progession.
    // size of area compared with original map covers increases with step distance as:
    // 1, 9, 25, 49 
    // (quadratic relationship) It follows that there will be a similar relactionship
    // between number of squares it's possible to visit and area covered. Hence:
    // quadratic progression, which we can get the first three values of by using 
    // part 1 code:
    for i in 0..=2 {
        progression.push(part1(gardens, starting_point, 65 + i*131));
    }
    // We know the relationship is quadratic, and the first three values in the sequence
    // Hence we can calculate the rest.
    progression[0] + progression[1]*max_grid_traversals + (max_grid_traversals*(max_grid_traversals -1)/2) * ((progression[2] - progression[1]) - (progression[1] - progression[0]))
}

fn parse_data(data: &str) -> (HashSet<Coordinate<i32>>, Coordinate<i32>) {
    let mut gardens = HashSet::new();
    let mut start = Coordinate{x:0, y:0};
    for (y, line) in data.lines().enumerate() {
        for (x,c) in line.chars().enumerate() {
            match c {
                '.' => {
                    gardens.insert(Coordinate{x: x as i32,y: y as i32});
                },
                'S' => {
                    start = Coordinate{x: x as i32,y: y as i32};
                    gardens.insert(Coordinate{x: x as i32,y: y as i32});
                }
                '#' => {}
                _ => unimplemented!()
            };
        }
    }

    (gardens,start)
    
}

#[cfg(test)]
mod tests {
    use super::*; 
    const EXAMPLE1:&str = 
"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn test_example1() {
        let (gardens, start) = parse_data(EXAMPLE1);
        assert_eq!(part1(&gardens, &start, 6), 16);
    }
}