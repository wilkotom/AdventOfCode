use std::fs::read_to_string;

fn main() {
    let data = read_to_string("./day16/input.txt")
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect::<Vec<i64>>();
    println!("Part 1: {}", &part1(data.clone())[0..8]);
    println!("Part 2: {}", &part2(data)[0..8]);

}

fn part2(data: Vec<i64>) -> String {
    let mut part2_data = data.iter().cycle().take(data.len() * 10000).copied().collect::<Vec<_>>();
    let mut offset = 0;
    for n in &part2_data[0..7] {
        offset *= 10;
        offset += n;
    }
    part2_data = part2_data[offset as usize..].to_vec();

    for _ in 0..100 {
        for i in (0..part2_data.len() -2).rev() {
            part2_data[i] = (part2_data[i] + part2_data[i+1] ) % 10;
        }
    }
    data.iter().map(|n| n.to_string()).collect::<String>()
}

fn part1(mut data: Vec<i64>) -> String {
    for _ in 0..100 {
        data = transform(data);
    }
    data.iter().map(|n| n.to_string()).collect::<String>()

}

fn transform(numbers: Vec<i64>) -> Vec<i64> {

    let pattern = [0, 1, 0, -1];
    let mut result = Vec::new();
    for (i, _) in numbers.iter().enumerate() {
        result.push(numbers.iter().enumerate().map(|(j,n)| n * pattern[((j+1)/(i+1)) % 4]).sum::<i64>().abs() % 10);
    }
    result
}


#[test]
fn test_12345678_one_phase() {
    let before=vec![1,2,3,4,5,6,7,8];
    let after = transform(before);
    assert_eq!(after,[4,8,2,2,6,1,5,8])
}


#[test]
fn test_12345678_four_phases() {
    let mut numbers = vec![1,2,3,4,5,6,7,8];
    for _ in 0..4 {
        numbers = transform(numbers);
    }
    assert_eq!(numbers,[0,1,0,2,9,4,9,8])
}

#[test]
fn test_80871224585914546619083218645595_100_phases() {
    let numbers = "80871224585914546619083218645595"
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect::<Vec<i64>>();
    let result = part1(numbers);
    assert_eq!(&result[0..8], "24176176");
}

#[test]
fn test_19617804207202209144916044189917_100_phases() {
    let numbers = "19617804207202209144916044189917"
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect::<Vec<i64>>();
    let result = part1(numbers);
    assert_eq!(&result[0..8], "73745418");
}

#[test]
fn test_69317163492948606335995924319873_100_phases() {
    let numbers = "69317163492948606335995924319873"
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect::<Vec<i64>>();
    let result = part1(numbers);
    assert_eq!(&result[0..8], "52432133");
}