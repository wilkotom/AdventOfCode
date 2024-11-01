use std::error::Error;
use aochelpers::get_daily_input;

#[derive(Eq,PartialEq,Debug)]
struct Hand {
    red: i32,
    green: i32,
    blue: i32
}

fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(2,2023)?;
    let answers = solution(&data);
    println!("Part 1: {}", answers.0);
    println!("Part 2: {}", answers.1);

    Ok(())
}

fn solution(data: &str) -> (i32,i32) {
    let mut part1_answer  = 0;
    let mut answer  = 0;
    for line in data.split('\n') {
        let mut words = line.split(": ");
        let game_id = words.next().unwrap()
            .strip_prefix("Game ").unwrap().parse::<i32>().unwrap();
        let mut min_cubes = Hand{red: 0, blue:0, green: 0};
        for hand in words.next().unwrap().split("; ") {
            let hand = parse_hand(hand);
            min_cubes.red = min_cubes.red.max(hand.red);
            min_cubes.blue = min_cubes.blue.max(hand.blue);
            min_cubes.green = min_cubes.green.max(hand.green);
        }
        if min_cubes.red <=12 && min_cubes.green <=13 && min_cubes.blue <= 14 {
            part1_answer += game_id;
        }
        answer += min_cubes.red * min_cubes.blue * min_cubes.green

    }
    (part1_answer, answer)
}


fn parse_hand(hand_str: &str) -> Hand {
    let mut hand = Hand{ red: 0, green: 0, blue: 0 };
    for cubes in hand_str.split(", ") {
        let mut colours = cubes.split(' ');
        let count = colours.next().unwrap().parse::<i32>().unwrap();
        match colours.next() {
            Some("red") => {
                hand.red += count
            },
            Some("blue") => {
                hand.blue += count
            },
            Some("green") => {
                hand.green += count
            },
            Some(_) | None => unimplemented!(),
        }
    }
    hand
}


#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";


    #[test]
    fn test_parse_hand() {
        assert_eq!(parse_hand("3 blue, 4 red"), Hand{red:4, blue: 3, green:0});
        assert_eq!(parse_hand("8 green, 6 blue, 20 red"), Hand{red:20, blue: 6, green:8});
    }

    #[test]
    fn test_part1() {
        assert_eq!(solution(&DATA).0, 8)
    }


    #[test]
    fn test_part2() {
        assert_eq!(solution(&DATA).1, 2286)
    }
}