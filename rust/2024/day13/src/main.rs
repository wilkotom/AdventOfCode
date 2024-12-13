use std::error::Error;
use aochelpers::{get_daily_input, Coordinate};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct GrabMachine {
    button_a: Coordinate<isize>,
    button_b: Coordinate<isize>,
    prize: Coordinate<isize>
}

fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(13,2024)?;
    let mut parsed: Vec<GrabMachine> = parse_data(&data);
    let result = parsed.iter().filter_map(|m| solve_equation(*m)).sum::<isize>();
    println!("Part 1: {}", result);
    parsed.iter_mut().for_each(|m| m.prize += Coordinate{x: 10000000000000, y: 10000000000000});
    let result = parsed.iter().filter_map(|m| solve_equation(*m)).sum::<isize>();
    println!("Part 2: {}", result);
    Ok(())
}

// Eurgh. Parsing at 5am is no fun
fn parse_data(data: &str) -> Vec<GrabMachine> {
    let mut results = Vec::new();
    for section in data.split("\n\n") {
        let mut lines = section.lines();
        let mut a_line = lines.next().unwrap().split(',');
        let ax = a_line.next().unwrap()[12..].parse().unwrap();
        let ay = a_line.next().unwrap()[3..].parse().unwrap();
        let mut b_line = lines.next().unwrap().split(',');
        let bx = b_line.next().unwrap()[12..].parse().unwrap();
        let by = b_line.next().unwrap()[3..].parse().unwrap();
        let mut prize_line = lines.next().unwrap().split(',');
        let px = prize_line.next().unwrap()[9..].parse().unwrap();
        let py = prize_line.next().unwrap()[3..].parse().unwrap();
        results.push(GrabMachine{
            button_a: Coordinate{x: ax, y: ay},
            button_b: Coordinate{x: bx, y: by},
            prize: {Coordinate{x: px, y: py}}
        });
    }
    results
}

fn solve_equation(machine: GrabMachine) -> Option<isize> {
    let a_presses =( machine.prize.y * machine.button_b.x - machine.prize.x * machine.button_b.y) / 
            (machine.button_a.y * machine.button_b.x - machine.button_a.x * machine.button_b.y);
    let b_presses = (machine.prize.x - a_presses * machine.button_a.x) / machine.button_b.x;
    if a_presses * machine.button_a.x + b_presses * machine.button_b.x != machine.prize.x || a_presses * machine.button_a.y + b_presses * machine.button_b.y != machine.prize.y {
        return None;
    }

    Some(3* a_presses + b_presses)
}


#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_parser() {
        let parsed = parse_data(DATA);
        assert_eq!(parsed, vec![
            GrabMachine{
            button_a:Coordinate { x: 94, y: 34 }, 
            button_b: Coordinate{x: 22, y:67}, 
            prize: Coordinate{x: 8400, y: 5400}
        },
        GrabMachine{
            button_a:Coordinate { x: 26, y: 66 }, 
            button_b: Coordinate{x: 67, y:21}, 
            prize: Coordinate{x: 12748, y: 12176}
        },
        GrabMachine{
            button_a:Coordinate { x: 17, y: 86 }, 
            button_b: Coordinate{x: 84, y:37}, 
            prize: Coordinate{x: 7870, y: 6450}
        },
        GrabMachine{
            button_a:Coordinate { x: 69, y: 23 }, 
            button_b: Coordinate{x: 27, y:71}, 
            prize: Coordinate{x: 18641, y: 10279}
        }
        ]);
    }

    #[test]
    fn test_part1() {
        
        assert_eq!(solve_equation(GrabMachine{
            button_a: Coordinate{ x: 94, y: 34 }, 
            button_b: Coordinate{x: 22, y:67}, 
            prize: Coordinate{x: 8400, y: 5400}
        }), Some(280));

        assert_eq!(solve_equation(GrabMachine{
            button_a:Coordinate { x: 26, y: 66 }, 
            button_b: Coordinate{x: 67, y:21}, 
            prize: Coordinate{x: 12748, y: 12176}
        }), None);

        let parsed: Vec<GrabMachine> = parse_data(DATA);
        let result = parsed.iter().map(|m| solve_equation(*m).unwrap_or(0)).sum::<isize>();
        assert_eq!(result, 480);

    }

    #[test]
    fn test_part2() {
    }
}