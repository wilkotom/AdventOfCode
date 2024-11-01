use std::{error::Error, collections::{HashMap, HashSet, VecDeque}};
use aochelpers::{get_daily_input, Coordinate, Direction, ScoredItem};

type PipeLookup = HashMap<Coordinate<i32>, (Direction, Direction)>;

fn part2(circuit: &HashSet<Coordinate<i32>>, arena: &HashMap<Coordinate<i32>, (Direction, Direction)>) -> i32 {

    let boundary = Coordinate{
        x: arena.keys().map(|c: &Coordinate<i32>| c.x).max().unwrap_or(0), 
        y: arena.keys().map(|c| c.y).max().unwrap_or(0)};

    let mut visited: HashSet::<Coordinate<i32>> = HashSet::new();
    let mut next_squares = VecDeque::new();
    // Flood fill the grid starting at (maxX, maxY) - we know this square will be unconnected to the 
    // circuit as it's part of the padding added at parse time
    next_squares.push_back(boundary);
    while let Some(next_square) = next_squares.pop_front() {
        if visited.contains(&next_square) || circuit.contains(&next_square) {
            continue;
        }
        visited.insert(next_square);
        for neighbour in next_square.neighbours() {
            if neighbour.x >=0 && neighbour.x <= boundary.x+1 && 
                neighbour.y >= 0 && neighbour.y <= boundary.y+1 {
                    next_squares.push_back(neighbour);
            }
        }
    }
    // Part 2 answer is the number of even-numbered squares in the arena that weren't
    // visited by either the flood fill or the circuit
    (0..boundary.y).step_by(2).map(|y: i32| (0..boundary.x).step_by(2)
        .filter(|x| !visited.contains(&Coordinate {x: *x, y}) && !circuit.contains(&Coordinate {x: *x, y}))
        .count() as i32).sum()

}

fn traverse_pipes(arena: &HashMap<Coordinate<i32>, (Direction, Direction)>, start: &Coordinate<i32>) -> (i32, HashSet<Coordinate<i32>>) {

    let mut visited: HashSet::<Coordinate<i32>> = HashSet::new();
    let mut next_squares = VecDeque::new();
    let mut max_distance = 0;
    next_squares.push_back(ScoredItem{cost: 0, item: *start});
    while let Some(current_state) = next_squares.pop_front() {
        if visited.contains(&current_state.item) {
            continue;
        }
        max_distance = current_state.cost;
        let leading_pipes = arena.get(&current_state.item).unwrap();
        let pipe = current_state.item.neighbour(leading_pipes.0);
        if !visited.contains(&pipe) {
            next_squares.push_back(ScoredItem { cost: current_state.cost +1, item: pipe});
        } 
        let pipe = current_state.item.neighbour(leading_pipes.1);
        if !visited.contains(&pipe) {
            next_squares.push_back(ScoredItem { cost: current_state.cost +1, item: pipe});
        } 
        visited.insert(current_state.item);
    }
    (max_distance, visited)
}

fn parse_data(data: &str) -> (PipeLookup, Coordinate<i32>){
    let mut starting_point = Coordinate{x:0_i32, y:0};
    let mut arena = HashMap::new();
    // each character in the input grid is converted to a 2x2 tile 
    // This means there's a gap that we can flood fill through for part 2
    for (y,line) in data.split('\n').enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '|' => {arena.insert(Coordinate{x: x as i32 *2 , y: y as i32 *2 },(Direction::North, Direction::South));
                        arena.insert(Coordinate{x: x as i32 *2 , y: y as i32 *2 +1},(Direction::North, Direction::South));},

                '-' => {arena.insert(Coordinate{x: x as i32 *2, y: y as i32 *2 }, (Direction::East, Direction::West));
                        arena.insert(Coordinate{x: x as i32 *2 +1, y: y as i32 *2}, (Direction::East, Direction::West));},

                'L' => {arena.insert(Coordinate{x: x as i32 *2, y: y as i32*2},(Direction::North, Direction::East));
                        arena.insert(Coordinate{x: x as i32 *2 +1, y: y as i32*2}, (Direction::East, Direction::West));},

                'J' => {arena.insert(Coordinate{x: x as i32 *2, y: y as i32 *2},(Direction::North, Direction::West));},

                '7' => {arena.insert(Coordinate{x: x as i32 *2, y: y as i32*2 },(Direction::South, Direction::West));
                        arena.insert(Coordinate{x: x as i32 *2, y: y as i32 *2 +1},(Direction::North, Direction::South));},

                'F' => {arena.insert(Coordinate{x: x as i32 *2, y: y as i32 *2 },(Direction::South, Direction::East));
                        arena.insert(Coordinate{x: x as i32 *2 +1, y: y as i32 *2}, (Direction::East, Direction::West));
                        arena.insert(Coordinate{x: x as i32 *2 , y: y as i32*2 +1},(Direction::North, Direction::South));},
                'S' => starting_point = Coordinate{x: x as i32 *2, y: y as i32 *2},
                '.' => {},
                _ => panic!()
            };
        }
    }

    // Work out what sort of tile S is
    let north_pipe = arena.get(&starting_point.neighbour(Direction::North)).unwrap_or(&(Direction::East, Direction::West));
    let east_pipe = arena.get(&(starting_point.neighbour(Direction::East) + Coordinate{x:1, y:0})).unwrap_or(&(Direction::North, Direction::South));
    let west_pipe = arena.get(&starting_point.neighbour(Direction::West)).unwrap_or(&(Direction::North, Direction::South));

    if north_pipe.0 == Direction::South || north_pipe.1 == Direction::South {
        //  Either J, L or |
        if west_pipe.0 == Direction::East || west_pipe.1 == Direction::East {
            // "J"
            arena.insert(starting_point,(Direction::North, Direction::West));
        }  else if east_pipe.0 == Direction::West || east_pipe.1 == Direction::West {
            // "L"
            arena.insert(starting_point,(Direction::North, Direction::East));
            arena.insert(starting_point + Coordinate { x: 1, y: 0 },  (Direction::East, Direction::West));
        } else {
            // |
            arena.insert(starting_point,(Direction::North, Direction::South));
            arena.insert(starting_point + Coordinate { x: 0, y: 1 },  (Direction::North, Direction::South));
        }

    } else if (west_pipe.0 == Direction::East || west_pipe.1 == Direction::East) && east_pipe.1 == Direction::West{
        // -
        arena.insert(starting_point,(Direction::East, Direction::West));
        arena.insert(starting_point + Coordinate { x: 1, y: 0 },(Direction::East, Direction::West));
    } else if west_pipe.0 == Direction::East || west_pipe.1 == Direction::East {
        // 7
        arena.insert(starting_point,(Direction::South, Direction::West));
        arena.insert(starting_point + Coordinate { x: 0, y: 1 },(Direction::North, Direction::South));
    } else {
        // F
        arena.insert(starting_point,(Direction::South, Direction::East));
        arena.insert(starting_point + Coordinate { x: 0, y: 1 },(Direction::North, Direction::South));
        arena.insert(starting_point + Coordinate { x: 1, y: 0 },(Direction::East, Direction::West));
    }
    (arena, starting_point)
    
}


fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(10,2023)?;
    let (arena, start) = parse_data(&data);
    let part1_arena = arena.iter()
        .filter(|e| e.0.x %2 == 0 && e.0.y %2 == 0)
        .map(|e| (Coordinate{x: e.0.x/2, y: e.0.y /2}, *e.1)).collect::<HashMap<Coordinate<i32>, _>>();
    let (part1, _) = traverse_pipes(&part1_arena, &Coordinate { x: start.x /2, y: start.y /2 });
    println!("Part 1: {}",part1);
    let (arena, start) = parse_data(&data);
    let (_, circuit) = traverse_pipes(&arena, &start);
    println!("Part 2: {}", part2(&circuit, &arena));

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let simple_loop = ".....\n.S-7.\n.|.|.\n.L-J.\n.....";
        
        let (arena, starting_point) = parse_data(simple_loop);
        let arena = arena.iter()
        .filter(|e: &(&Coordinate<i32>, &(Direction, Direction))| e.0.x %2 == 0 && e.0.y %2 == 0)
        .map(|e| (Coordinate{x: e.0.x/2, y: e.0.y /2}, *e.1)).collect::<HashMap<Coordinate<i32>, _>>();
        let starting_point = Coordinate { x: starting_point.x /2, y: starting_point.y /2 };

        assert_eq!(starting_point, Coordinate{x:1,y:1});
        assert_eq!(arena.get(&starting_point), Some(&(Direction::South, Direction::East)))
    }

    #[test]
    fn test_pipe_traverse() {
        let simple_loop = ".....\n.S-7.\n.|.|.\n.L-J.\n.....";
        let (arena, starting_point) = parse_data(simple_loop);
        assert_eq!(traverse_pipes(&arena,&starting_point).0, 8);
        let complex_loop = "..F7.\n.FJ|.\nSJ.L7\n|F--J\nLJ...";
        let (arena, starting_point) = parse_data(complex_loop);

        assert_eq!(traverse_pipes(&arena,&starting_point).0, 16);

    }
    #[test]
    fn test_part2() {
        let grid = 
"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        let (arena, starting_point) = parse_data(grid);
        let (_, circuit) = traverse_pipes(&arena, &starting_point);
        assert_eq!(part2(&circuit, &arena), 4);
        let grid = 
"...........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";
        let (arena, starting_point) = parse_data(grid);
        let (_, circuit) = traverse_pipes(&arena, &starting_point);
        assert_eq!(part2(&circuit, &arena), 4);

    }
}