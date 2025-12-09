use std::{collections::HashSet, error::Error};
use aochelpers::{get_daily_input, Coordinate};

#[derive(Debug,Clone, Copy, PartialEq, PartialOrd)]
struct Robot {
    position: Coordinate<i32>,
    velocity: Coordinate<i32>
}

fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(14,2024)?;
    let robots = parse_data(&data);
    println!("{}", part1(&robots, Coordinate { x: 101, y: 103 }, 100));
    println!("{}", part2(&robots, Coordinate { x: 101, y: 103 }));

    Ok(())
}

fn part1(robots: &Vec<Robot>, arena_dimensions: Coordinate<i32>, seconds: i32) -> usize {
    let mut quadrant_counts = [0;4];
    for robot in robots {
        let final_pos = Coordinate{
                        x:  (robot.position.x + (robot.velocity.x * seconds)).rem_euclid(arena_dimensions.x), 
                        y: (robot.position.y + (robot.velocity.y * seconds)).rem_euclid(arena_dimensions.y),
                    };
        if final_pos.x < (arena_dimensions.x -1) /2  && final_pos.y < (arena_dimensions.y -1) /2 {
            quadrant_counts[0] +=1
        } else if final_pos.x > (arena_dimensions.x -1) /2  && final_pos.y < (arena_dimensions.y -1) /2 {
            quadrant_counts[1] +=1
        } else if final_pos.x < (arena_dimensions.x -1) /2  && final_pos.y > (arena_dimensions.y -1) /2 {
            quadrant_counts[2] +=1
        } else if final_pos.x > (arena_dimensions.x -1) /2  && final_pos.y > (arena_dimensions.y -1) /2 {
            quadrant_counts[3] +=1
        } 
    }
    quadrant_counts.iter().product()
}

fn part2(robots: &Vec<Robot>,  arena_dimensions: Coordinate<i32>) -> i32 {
    let mut seconds = 0;
    loop {
        let mut occupied = HashSet::new();
        for robot in robots {
            occupied.insert(Coordinate{
                x:  (robot.position.x + (robot.velocity.x * seconds)).rem_euclid(arena_dimensions.x), 
                y: (robot.position.y + (robot.velocity.y * seconds)).rem_euclid(arena_dimensions.y),
            });
        }

        let connected_count = occupied.iter().filter(|c| c.neighbours().any(|n| occupied.contains(&n))).count();
        if connected_count >= robots.len() / 2 {
            print_robots(occupied, arena_dimensions);
            return seconds;
        }
        seconds +=1;
    }
}


fn print_robots(robots: HashSet<Coordinate<i32>>, dimensions:Coordinate<i32>) {
    for y in 0..dimensions.y {
        for x in 0..dimensions.x {
            print!("{}", if robots.contains(&Coordinate{x,y}) {"#"} else {" "});
        }
        println!();
    }
    

}
fn parse_data(data: &str) -> Vec<Robot> {
    let mut robots = Vec::new();
    for line in data.lines() {
        let mut sections = line.split_ascii_whitespace();
        let mut position = sections.next().unwrap()[2..].split(',').map(|v|v.parse().unwrap());
        let mut velocity = sections.next().unwrap()[2..].split(',').map(|v|v.parse().unwrap());
        robots.push(Robot{
            position: Coordinate{x: position.next().unwrap(), y: position.next().unwrap()},
            velocity: Coordinate{x: velocity.next().unwrap(), y: velocity.next().unwrap()}
        })
    }
    robots
}


#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test_part1() {
        let data = parse_data(TESTDATA);
        assert_eq!(part1(&data, Coordinate { x: 11, y: 7 }, 100), 12);
    }

    #[test]
    fn test_part2() {
    }
}