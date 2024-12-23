use std::error::Error;
use aochelpers::get_daily_input;

fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(03,2024)?;
    println!("Part 1: {}",solve(&data, false));
    println!("Part 2: {}",solve(&data, true));

    Ok(())
}

fn solve(instructions: &str, part2: bool) -> i64 {
    let mut res:i64 = 0;
    let mut left_ptr =0;
    let mut right_ptr;
    while left_ptr < instructions.len(){
        while left_ptr < instructions.len() && !instructions[left_ptr..].starts_with("mul(") {
            if instructions[left_ptr..].starts_with("don't()") && part2 {
                while left_ptr < instructions.len() && !instructions[left_ptr..].starts_with("do()") {
                    left_ptr +=1;
                }
            }
            left_ptr+=1;
        }
        left_ptr+=4;
        right_ptr = left_ptr;
        while right_ptr<instructions.len() &&  !instructions[right_ptr..].starts_with(")"){
            if instructions[right_ptr..].starts_with("mul(") {
                left_ptr = right_ptr +4;   
                right_ptr = left_ptr;
            }
            right_ptr +=1;
        }
        if right_ptr < instructions.len() {
            res += &instructions[left_ptr..right_ptr].split(",").map(|v| v.parse::<i64>().unwrap_or(0)).product();
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(solve("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))", false), 161);
        
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))", true), 48);

    }
}
