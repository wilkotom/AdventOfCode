use aochelpers::get_daily_input;
use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(1,2025)?;
    let (part1, part2) = solve(&data);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn solve(data: &str) -> (i32, i32) {
    let mut pos = 50;
    let mut part1  = 0;
    let mut answer = 0;
    for line in data.lines() {
        let turn = line[1..].parse::<i32>().unwrap();
        answer += turn / 100;
        if line.starts_with('L') {
            if turn % 100 > pos && pos != 0  {
                answer +=1;
            }
            pos -= turn;
            pos = pos.rem_euclid(100);
        } else {
            if pos + turn % 100 > 100 {
                answer +=1;
            }
            pos = (pos + turn) % 100;
        }
        if pos == 0 {
            part1 += 1;
            answer += 1;
        }
    }
    (part1, answer)
}

#[cfg(test)]
mod tests {

    const TESTDATA: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
    use super::*;
    // #[test]
    // fn test_p1() {
    //     assert_eq!(part1(TESTDATA),3);
    // }
    #[test]
        fn test_p2() {
        assert_eq!(solve(TESTDATA),(3,6));
        assert_eq!(solve("R1000"),(0,10));
        assert_eq!(solve("L1000"),(0,10));

    }

}