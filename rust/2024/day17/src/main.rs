use std::error::Error;
use aochelpers::get_daily_input;

fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(17,2024)?;
    let mut computer = parse_data(&data);
    let output = computer.run_program();
    println!("Part 1: {}", &output[..].iter().map(|c| c.to_string()).collect::<Vec<_>>().join(","));
    println!("Part 2: {}", part2(&mut computer));
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
struct NotAnIntCodeComputer {
    a: u64,
    b: u64,
    c: u64,
    ptr: usize,
    program:Vec<u64>
}

impl NotAnIntCodeComputer {
    fn run_program(&mut self) -> Vec<u64> {
        let mut output = Vec::new();
        while self.ptr < self.program.len() {
            let instruction = self.program[self.ptr];
            let literal = self.program[self.ptr +1];
            let combo = match literal {
                v if v < 4 => v,
                4 => self.a,
                5 => self.b,
                6 => self.c,
                v => v
            };
            match instruction {
                0 => {
                    // adv
                    self.a >>= combo;
                    self.ptr +=2;
                },
                1 => {
                    //bxl
                    self.b ^= literal;
                    self.ptr +=2;
                }
                2 => {
                    //bst
                    self.b = combo % 8;
                    self.ptr +=2;

                }
                3 => {
                    //jnz
                    if self.a == 0 {
                        self.ptr +=2;
                    } else {
                        self.ptr = combo as usize;
                    }
                }
                4 => {
                    // bxc
                    self.b ^= self.c;
                    self.ptr +=2;
                }
                5 => {
                    //out
                    output.push(combo % 8);
                    self.ptr +=2;
                }
                6 => {
                    //bdv
                    self.b = self.a >> combo;
                    self.ptr +=2;
                }
                7 => {
                    //cdv
                    self.c = self.a >> combo;
                    self.ptr +=2;
                }
                _ => unimplemented!()
            }   
        }
        output
    }
}

fn parse_data(data: &str) -> NotAnIntCodeComputer {
    let mut sections = data.split("\n\n");
    let mut registers = sections.next().unwrap().lines();
    let a = registers.next().unwrap()[12..].parse::<u64>().unwrap();
    let b = registers.next().unwrap()[12..].parse::<u64>().unwrap();
    let c = registers.next().unwrap()[12..].parse::<u64>().unwrap();
    let program = sections.next().unwrap()[9..].split(",").map(|v| v.parse().unwrap()).collect();
    NotAnIntCodeComputer{
        a,
        b,
        c,
        ptr: 0,
        program
    }
}

fn part2(computer: &mut NotAnIntCodeComputer) -> u64 {
    let mut potential_matches = Vec::new();
    potential_matches.push((0,0));
    while let Some((match_count, guess) ) = potential_matches.pop() {
        if match_count == computer.program.len() {
            return guess;
        }
        for i in (0..=7).rev() {
            let next_guess = guess * 8 +i; 
            computer.a = next_guess;
            computer.b = 0;
            computer.c = 0;
            computer.ptr = 0;
            let output = computer.run_program();
            if computer.program[computer.program.len() - (match_count+1)] == output[0] {
                potential_matches.push((match_count +1, next_guess));
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let computer = parse_data("Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0");
        assert_eq!(computer, NotAnIntCodeComputer{a: 729, b: 0, c: 0, ptr: 0, program: vec![0,1,5,4,3,0]})
    }

    #[test]
    fn test_part1_ex1() {
        let mut computer = NotAnIntCodeComputer{
            a: 0, b:0, c:9,
            program: vec![2,6],
            ptr:0
        };
        computer.run_program();
        assert_eq!(computer.b, 1);
    }

    #[test]
    fn test_part1_ex2() {
        let mut computer = NotAnIntCodeComputer{
            a: 10, b:0, c:0,
            program: vec![5,0,5,1,5,4],
            ptr:0
        };
        let output = computer.run_program();
        assert_eq!(output, vec![0,1,2]);
    }

    #[test]
    fn test_part1_ex3() {
        let mut computer = NotAnIntCodeComputer{
            a: 2024, b:0, c:0,
            program: vec![0,1,5,4,3,0],
            ptr:0
        };
        let output = computer.run_program();
        assert_eq!(output, vec![4,2,5,6,7,7,7,7,3,1,0]);
        assert_eq!(computer.a, 0);
    }


    #[test]
    fn test_part1_ex4() {
        let mut computer = NotAnIntCodeComputer{
            a: 0, b:29, c:0,
            program: vec![1,7],
            ptr:0
        };
        computer.run_program();
        assert_eq!(computer.b, 26);
    }

    #[test]
    fn test_part1_ex5() {
        let mut computer = NotAnIntCodeComputer{
            a: 0, b:2024, c:43690,
            program: vec![4,0],
            ptr:0
        };
        computer.run_program();
        assert_eq!(computer.b, 44354);
    }

    #[test]
    fn test_part1_ex6() {
        let mut computer = NotAnIntCodeComputer{
            a: 729, b:0, c:0,
            program: vec![0,1,5,4,3,0],
            ptr:0
        };
        assert_eq!(computer.run_program(), [4, 6, 3, 5, 6, 3, 5, 2, 1, 0] )
    }

}