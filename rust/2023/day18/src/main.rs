use std::error::Error;
use aochelpers::{get_daily_input, Direction, Coordinate};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Instruction {
    direction: Direction,
    distance: i128,
}


fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(18,2023)?;
    let instructions = parse_data(&data);
    println!("Part 1: {},", answer(&instructions.0));
    println!("Part 2: {},", answer(&instructions.1));

    Ok(())
}

fn answer(instructions: &Vec<Instruction>) -> i128 {
    
    let mut position = Coordinate{x:0, y:0};
    let mut locations = Vec::new();
    let mut perimeter = 0;
    for instruction in instructions {
        locations.push(position);
        perimeter += instruction.distance;
        let delta = match instruction.direction {
            Direction::North => {Coordinate{x:0, y:-instruction.distance}},
            Direction::East => {Coordinate{x:instruction.distance, y:0}},
            Direction::South => {Coordinate{x:0, y:instruction.distance}},
            Direction::West => Coordinate{x:-instruction.distance, y:0},
            _ => unimplemented!()};
        position += delta;
    }
    // Shoelace Theorem - gives area including perimeter
    // However this is an overestimate, as we want the area bounded by
    // the centre of each 1x1 block, not the whole block
    let mut area = 0;
    for i in 0..locations.len()-1 {
        area += locations[i].x * locations[i+1].y - locations[i].y * locations[i+1].x;
        
    }
    area += locations[locations.len()-1].x * locations[0].y - locations[locations.len()-1].y * locations[0].x;

    if area < 0 {
        area = 0 - area
    }
    area /= 2;
    // Correction factor: 
    // We only count perimeter squares on the left or top
    // ie, half, plus one
    area + (perimeter / 2) + 1
   }



fn parse_data(data: &str) -> (Vec<Instruction>, Vec<Instruction>) {
    let mut part1_instructions: Vec<Instruction> = Vec::new();
    let mut part2_instructions: Vec<Instruction> = Vec::new();

    for line in data.lines() {
        let mut tokens = line.split(' ');
        let direction= match tokens.next() {
            Some("R") => Direction::East,
            Some("L") => Direction::West,
            Some("U") => Direction::North,
            Some("D") => Direction::South,
            _ => unimplemented!()
        };
        let distance = tokens.next().unwrap().parse::<i128>().unwrap();
        let part2_in_disguise = tokens.next().unwrap().to_string();
        part1_instructions.push(Instruction{direction, distance});

        let p2_distance = i128::from_str_radix(&part2_in_disguise[2..7], 16).unwrap();
        let p2_direction = match part2_in_disguise.chars().nth(7) {
            Some('0') => Direction::East,
            Some('1') => Direction::South,
            Some('2') => Direction::West,
            Some('3') => Direction::North,
            _ => unimplemented!()
        };
        part2_instructions.push(Instruction{direction: p2_direction, distance: p2_distance});
    }

    (part1_instructions, part2_instructions)
}

#[cfg(test)]
mod tests {

    use super::*; 
    const DATA: &str = 
"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn test_parser() {
        assert_eq!(parse_data("R 2 (#59c680)\nD 2 (#411b91)\nL 5 (#8ceee2)\nU 2 (#caa173)"), 
            (vec![
                Instruction{direction: Direction::East, distance:2}, 
                Instruction{direction: Direction::South, distance:2}, 
                Instruction{direction: Direction::West, distance:5}, 
                Instruction{direction: Direction::North, distance:2}, 
            ], 
        vec![Instruction { direction: Direction::East, distance: 367720}, 
             Instruction { direction: Direction::South, distance: 266681}, 
             Instruction { direction: Direction::West, distance: 577262}, 
             Instruction { direction: Direction::North, distance: 829975}]))
    }

    #[test]
    fn test_answers() {
        let( p1_instructions, p2_instructions) = parse_data(DATA);
        assert_eq!(answer(&p1_instructions), 62);
        assert_eq!(answer(&p2_instructions), 952408144115)

    }
}