use std::{error::Error, collections::{HashMap, hash_map::DefaultHasher}, hash::{Hash, Hasher}};
use aochelpers::{get_daily_input, Coordinate, Direction};

#[derive(Debug,Clone,Copy, PartialEq, Eq)]
enum Rock {
    Round,
    Square
}

fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(14,2023)?;
    let mut arena: HashMap<Coordinate<i64>, Rock> = parse_data(&data);
    let boundary: Coordinate<i64> = Coordinate{ 
        x: arena.keys().map(|c| c.x).max().unwrap(),
        y: arena.keys().map(|c| c.y).max().unwrap()
    }; 
    
    roll_rocks(&mut arena, &boundary, Direction::North);
    println!("Part 1: {}",support_beam_load(&arena, &boundary));

    let mut cycles = 0;
    let mut seen : HashMap<u64, i128> = HashMap::new(); 
    let mut state = hash_arena(&arena, &boundary);
    while !seen.contains_key(&state) && cycles < 200{
        seen.insert(state, cycles);
        cycle_rocks(&mut arena, &boundary);
        cycles +=1;
        state = hash_arena(&arena, &boundary);
    }

    let cycles_remaining = (1_000_000 - cycles) %(cycles -seen.get(&state).unwrap());
     for _ in 0..cycles_remaining {
        cycle_rocks(&mut arena, &boundary);
    }
    println!("Part 2: {}",support_beam_load(&arena, &boundary));

    Ok(())
}

fn parse_data(data: &str) -> HashMap<Coordinate<i64>,Rock> {
    let mut arena = HashMap::new();
    for (y, line) in data.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                'O' => {arena.insert(Coordinate{x:x as i64,y:y as i64}, Rock::Round);},
                '#' => {arena.insert(Coordinate{x:x as i64,y:y as i64}, Rock::Square);},
                '.' => {},
                _ => unimplemented!()
            }
        }
    }
    arena
}

fn cycle_rocks(arena: &mut HashMap<Coordinate<i64>, Rock>, boundary: &Coordinate<i64>) {
    roll_rocks(arena, boundary, Direction::North);
    roll_rocks(arena, boundary, Direction::West);
    roll_rocks(arena, boundary, Direction::South);
    roll_rocks(arena, boundary, Direction::East);
}

fn roll_rocks(arena: &mut HashMap<Coordinate<i64>, Rock>, boundary: &Coordinate<i64>, direction: Direction) {
    let rocks_in_order = match direction {
        Direction::North => (0..=boundary.y).flat_map(|y| (0..=boundary.x).map(move |x| Coordinate{x,y})).collect::<Vec<_>>(),
        Direction::South => (0..=boundary.y).rev().flat_map(|y| (0..=boundary.x).map(move |x| Coordinate{x,y})).collect::<Vec<_>>(),
        Direction::East => (0..=boundary.x).rev().flat_map(|x| (0..=boundary.y).map(move |y| Coordinate{x,y})).collect::<Vec<_>>(),
        Direction::West => (0..=boundary.x).flat_map(|x| (0..=boundary.y).map(move |y| Coordinate{x,y})).collect::<Vec<_>>(),
        _ => unimplemented!()
    };

    for rock in rocks_in_order {
        if let Some(&Rock::Round) = arena.get(&rock) {
            let mut target_loc = rock;
            while ((target_loc.x > 0 && direction == Direction::West) ||  (target_loc.x < boundary.x && direction == Direction::East) || 
                    (target_loc.y > 0 && direction == Direction::North) ||  (target_loc.y < boundary.y && direction == Direction::South)) && 
                    !arena.contains_key(&target_loc.neighbour(direction)) {
                target_loc = target_loc.neighbour(direction)
            }
            if target_loc != rock {
                arena.insert(target_loc, Rock::Round);
                arena.remove(&rock);
            }
        }
    }
}

fn support_beam_load(arena: &HashMap<Coordinate<i64>, Rock>, boundary: &Coordinate<i64>) -> i64 {
    arena.iter().filter_map(|(c, r)| if *r == Rock::Round {Some(boundary.y +1 - c.y)} else {None}).sum()
}

fn hash_arena(arena: &HashMap<Coordinate<i64>, Rock>,  boundary: &Coordinate<i64>) -> u64 {
    let mut result = Vec::new();
    for y in 0..=boundary.y {
        let mut row: i128 = 0;
        for x in 0..=boundary.x {
            row *= 2;
            if arena.contains_key(&Coordinate { x, y}) {
                row +=1;
            }
        } 
        result.push(row);
    }
    let mut hasher = DefaultHasher::new();
    result.hash(&mut hasher);
    hasher.finish()
    
}


#[cfg(test)]
mod tests {

    use super::*; 

    const EXAMPLE : &str = 
"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

const EXAMPLE_ROLLED: &str = 
".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....";

    #[test]
    fn test_roller() {
        let mut arena: HashMap<Coordinate<i64>, Rock> = parse_data(EXAMPLE);
        let boundary = Coordinate{ x: arena.keys().map(|c| c.x).max().unwrap(),
            y : arena.keys().map(|c| c.y).max().unwrap()}; 
        cycle_rocks(&mut arena, &boundary);
        let cycle_one = parse_data(EXAMPLE_ROLLED);
        assert_eq!(arena, cycle_one);
    }
}


