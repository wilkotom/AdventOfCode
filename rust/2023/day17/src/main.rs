use std::{error::Error, collections::{HashMap, BinaryHeap}};
use aochelpers::{get_daily_input, Coordinate, Direction, ScoredItem, parse_number_grid};


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Crucible {
    facing: Direction,
    location: Coordinate<i32>,
    steps_taken: i32
}

fn main() -> Result<(), Box<dyn Error>>{
     let data = get_daily_input(17,2023)?;
    let grid: HashMap<Coordinate<i32>, i32> = parse_number_grid(&data);
    println!("{}",part1(&grid,0,3));
    println!("{}",part1(&grid,4,10));

    Ok(())
}

fn part1(grid: &HashMap<Coordinate<i32>,i32>, min_move: i32, max_move:i32) -> i32 {
    
    let mut repeated_states = HashMap::new();
    let mut unconsidered = BinaryHeap::new();

    let goal = Coordinate{ 
        x: grid.keys().map(|c| c.x).max().unwrap(),
        y: grid.keys().map(|c| c.y).max().unwrap()
    }; 

    unconsidered.push(ScoredItem{cost:0 , item: Crucible{facing: Direction::East, location: Coordinate { x:0, y:0}, steps_taken: 0}});
    unconsidered.push(ScoredItem{cost:0 , item: Crucible{facing: Direction::South, location: Coordinate { x:0, y:0}, steps_taken: 0}});


    while let Some(current_square)  = unconsidered.pop() {

        let best_for_square = repeated_states.get(&current_square.item).unwrap_or(&i32::MAX);
        if *best_for_square <= current_square.cost  {
            continue;
        }
        repeated_states.insert(current_square.item, current_square.cost);
        if current_square.item.location == goal  && current_square.item.steps_taken >=  min_move {
            return current_square.cost;
        } else if current_square.item.location == goal {
            continue;
        }
        if current_square.item.steps_taken +1 < max_move {
            let next_square = current_square.item.location.neighbour(current_square.item.facing);
            if grid.contains_key(&next_square) {
                unconsidered.push(
                ScoredItem{cost:current_square.cost + grid.get(&next_square).unwrap() , item: Crucible{
                    facing: current_square.item.facing, 
                    location: next_square, 
                    steps_taken: current_square.item.steps_taken +1,
                }});
            }
        }
        let forks = match current_square.item.facing {
            Direction::North | Direction::South => [Direction::East, Direction::West],
            Direction::East | Direction::West=> [Direction::North, Direction::South],
            _ => unimplemented!()
        };
        if current_square.item.steps_taken +1 >= min_move  {
            for next_direction in forks {
                let next_square = current_square.item.location.neighbour(next_direction);
                if grid.contains_key(&next_square) {
                    unconsidered.push(
                        ScoredItem{cost:current_square.cost + grid.get(&next_square).unwrap() , item: Crucible{
                            facing: next_direction, 
                            location: next_square, 
                            steps_taken: 0, 
                        },
                    });
                }
            } 
        }
    }
    0
}

#[cfg(test)]
mod tests {

    use super::*; 
    const DATA: &str = 
"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    #[test]
    fn test_checksum() {
        let grid: HashMap<Coordinate<i32>, i32> = parse_number_grid(DATA);
        assert_eq!(part1(&grid,0,3), 102);
        assert_eq!(part1(&grid,4,10), 94);

    }
}