use std::{collections::{BinaryHeap, HashSet}, error::Error};
use aochelpers::{get_daily_input, Coordinate, ScoredItem};

fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(18,2024)?;
    let falling: Vec<Coordinate<usize>> = parse_data(&data);
    println!("Part 1: {}",part1(&falling, Coordinate { x: 70, y: 70 }, 1024).unwrap());
    println!("Part 2: {}",part2(&falling, Coordinate { x: 70, y: 70 }));
    Ok(())
}

#[derive(Ord,PartialOrd,PartialEq,Eq, Debug, Hash, Copy, Clone)]
struct StepState {
    steps_taken: usize,
    position: Coordinate<usize>
}

fn part1(falling: &[Coordinate<usize>], goal: Coordinate<usize>, to_fall: usize) -> Option<usize> {
    let mut arena = HashSet::new();
    falling[..to_fall].iter().for_each(|c| {arena.insert(*c);});
    let mut unvisited = BinaryHeap::new();
    let mut visited = HashSet::new();
    let initial_state = ScoredItem{cost: goal.x + goal.y, item: StepState{steps_taken: 0, position: Coordinate{x:0,y:0}}};
    unvisited.push(initial_state);
    while let Some(state) = unvisited.pop() {
        if state.item.position == goal {
            return Some(state.item.steps_taken);
        }
        if visited.contains(&state.item.position) {
            continue;
        }
        visited.insert(state.item.position);
        for neighbour in state.item.position.checked_neighbours() {
            if !arena.contains(&neighbour) && neighbour.x <= goal.x && neighbour.y <= goal.y && ! visited.contains(&neighbour){
                unvisited.push(ScoredItem{cost: neighbour.manhattan_distance(&goal) + state.item.steps_taken +1, item: StepState{position: neighbour, steps_taken: state.item.steps_taken +1} });
            }
        }
    }
    None
}

fn part2(falling: &[Coordinate<usize>], goal: Coordinate<usize>) -> Coordinate<usize> {
    let mut inspection_point = falling.len() -1;
    let mut window_size = falling.len() /2;
    while window_size  > 0 {
        if part1(falling, goal, inspection_point).is_some() {
            inspection_point += window_size;
        } else {
            inspection_point -= window_size;
        }
        window_size /=2;
    }
    falling[inspection_point]
}

fn parse_data(data: &str) -> Vec<Coordinate<usize>> {
    data.lines().map(|l| {let mut s = l.split(",").map(|n| n.parse().unwrap()); Coordinate{x: s.next().unwrap(), y: s.next().unwrap()}}).collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const TESTDATA: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn test_part1() {
        let falling: Vec<Coordinate<usize>> = parse_data(TESTDATA);
        assert_eq!(part1(&falling, Coordinate { x: 6, y: 6 }, 12), Some(22));
        assert_eq!(part1(&falling, Coordinate { x: 6, y: 6 }, falling.len()), None);
    }

    #[test]
    fn test_part2() {
        let falling: Vec<Coordinate<usize>> = parse_data(TESTDATA);
        assert_eq!(part2(&falling, Coordinate { x: 6, y: 6 }), Coordinate{x:6, y:1})
    }
}