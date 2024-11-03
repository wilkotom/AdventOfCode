use std::error::Error;
use aochelpers::{get_daily_input, Coordinate};
use hashbrown::HashMap;

#[derive(Debug)]
enum Square {
     Number(i32),
     Symbol,
     Gear(i32)

}
fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(3,2023)?;
    let engine_details = parse_engine_schematic(&data);
    println!("Part 1: {}", engine_details.0);
    println!("Part 2: {}", engine_details.1);
    Ok(())
}

fn parse_engine_schematic(data: &str)-> (i32, i32) {
    let mut grid = parse_grid(data);
    let mut bounds = Coordinate{ x: 0, y: 0 };
    for (coord, _) in grid.iter() {
        bounds.x = bounds.x.max(coord.x);
        bounds.y = bounds.y.max(coord.y);
    }

    let mut part_numbers = 0;
    let mut gear_totals = 0;

    for y in 0..=bounds.y {
        let mut running_total = 0;
        let mut neighbour_detected = false;
        let mut gear = None;
        for x in 0..=bounds.x {
            match grid.get(&Coordinate{x,y}) {
                Some(Square::Number(n)) => {
                    running_total *= 10;
                    running_total += n;
                    if !neighbour_detected {
                        for coord in (Coordinate{x,y}).extended_neighbours().iter() {
                            match grid.get(coord) {
                            Some(Square::Symbol) => {
                                neighbour_detected = true;
                                break;
                            }
                            Some(Square::Gear(_)) => {
                                neighbour_detected = true;
                                gear = Some(*coord);
                                break;
                            }
                            _ => {},
                        }
                    }
                }
            },
                Some(Square::Symbol) |Some(Square::Gear(_)) | None => {
                    if running_total  >0 && neighbour_detected{
                        part_numbers += running_total;
                        neighbour_detected = false;
                    }
                    if let Some(c) = gear {
                        if let Some(Square::Gear(v)) = grid.get(&c) {
                            if *v == 0 {
                                grid.insert(c, Square::Gear(running_total));
                            } else {
                                gear_totals += running_total * v;
                            }
                        }
                    }
                    running_total = 0;
                    gear = None;
                }
            }
        }
        if neighbour_detected{ 
            part_numbers += running_total;
        }
        if let Some(c) = gear {
            if let Some(Square::Gear(v)) = grid.get(&c) {
                    gear_totals += running_total * v;
            }
        }
    }
    (part_numbers, gear_totals)
}


fn parse_grid(data: &str) -> HashMap<Coordinate<i32>, Square> {
 let mut output: HashMap<Coordinate<i32>, Square> = HashMap::new();

 for (y, line) in data.split('\n').enumerate() {
    for (x, c) in line.chars().enumerate() {
        if c.is_ascii_digit() {
            output.insert(Coordinate { x: x as i32, y: y as i32 }, Square::Number(c.to_digit(10).unwrap() as i32));
        } else if c == '*' {
            output.insert(Coordinate { x: x as i32, y: y as i32 }, Square::Gear(0));
    }else if c != '.' {
            output.insert(Coordinate { x: x as i32, y: y as i32 }, Square::Symbol);
        }
    }
 }


 output
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = 
"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";


    #[test]
    fn test_part1() {
        assert_eq!(parse_engine_schematic(DATA).0, 4361)
    }
}