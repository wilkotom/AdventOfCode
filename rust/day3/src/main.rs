use std::fs::File;
use std::io::{BufRead, BufReader, Error};


fn main() -> Result<(), Error> {
    let br = BufReader::new(File::open("./input.txt")?);
    let field: Vec<_>=  br.lines()
        .map(|l| l.expect("Unable to read line"))
        .collect();
    let part_1_score = check_slope(&field, 3,1);
    let part_2_score = check_slope(&field, 1,1) * check_slope(&field, 3,1) * check_slope(&field, 5,1) * check_slope(&field, 7,1) * check_slope(&field, 1,2);
    println!("Part 1 score: {}", part_1_score);
    println!("Part 2 score: {}", part_2_score);

    Ok(())
}


fn check_slope(field: &Vec<String>, xinc: usize, yinc: usize) -> usize{
    let mut x = 0;
    let mut y = 0;
    let mut count = 0;
    x = x % field[0].len();
    while y < field.len() {
        x = x % field[0].len();
        if field[y].chars().nth(x) == Some('#') {
            count +=1;
        }
        y = y + yinc;
        x = x + xinc;
    }
    count
}