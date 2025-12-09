use aochelpers::{get_daily_input, Coordinate, Rectangle};
use itertools::Itertools;
use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(9,2025)?;
    let tiles = parse_data(&data);
    println!("Part 1: {}", part1(&tiles));
    println!("Part 2: {}", part2(&tiles));
    Ok(())
}

fn part1(tiles: &[Coordinate<i64>]) -> i64 {
    tiles.iter().enumerate().map(
        |(i,t1)| tiles[i+1..].iter().map(
            |t2| ((t1.x - t2.x).abs() +1 )* ((t1.y - t2.y).abs()+1)).max().unwrap_or_default()
        ).max().unwrap_or_default()
}

fn part2(tiles: &[Coordinate<i64>]) -> i64 {
    let mut answer = 0;
    let edges = tiles.into_iter().circular_tuple_windows()
        .map(|(a,b)| Rectangle::new(*a, *b)).collect::<Vec<_>>();
    for (i, t1) in tiles.iter().enumerate() {
        for t2 in tiles[i+1..].iter() {
            // This is just to ensure I have top left and bottom right correctly defined.
            let rect = Rectangle::new(*t1, *t2);
            let inner_rect= Rectangle::new(
                    Coordinate{x: rect.top_left.x +1, y: rect.top_left.y+1},
                    Coordinate{x: rect.bottom_right.x - 1, y: rect.bottom_right.y-1});
            if edges.iter().all(|e|inner_rect.intersection(e).is_none()){
                answer = answer.max( ((t1.x - t2.x).abs() +1 )* ((t1.y - t2.y).abs()+1))
            } 
        }
    }   
    answer
}

fn parse_data(data: &str) -> Vec<Coordinate<i64>> {
    data.lines().map(|l| {
        let mut s = l.split(',');
        Coordinate{x: s.next().unwrap().parse().unwrap(), y: s.next().unwrap().parse().unwrap()}
    }).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    const TESTDATA: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_p1() {
        let tiles = parse_data(TESTDATA);
        assert_eq!(part1(&tiles), 50);
    }

    #[test]
    fn test_p2() {
        let tiles = parse_data(TESTDATA);
        assert_eq!(part2(&tiles), 24);
    }
}
