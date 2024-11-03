
fn main() {
    let data = std::fs::read_to_string("./input.txt").unwrap();
    let numbers = data.split("\n").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
    println!("Part 1: {}", part1(&numbers));
    println!("Part 2: {}", part2(&numbers));
}

fn part1(numbers: &[i32]) -> i32 {
    let mut last_value = numbers[0];
    let mut increment_count = 0;
    for num in &numbers[1..] {
        if *num > last_value {
            increment_count +=1
        }
        last_value = *num;
    }
    increment_count
}

fn part2(numbers: &[i32]) -> i32 {
    let mut sliding: Vec<i32> = Vec::new();
    let mut current_value = numbers[0]+ numbers[1] + numbers[2];
    sliding.push(current_value);
    for i in 1..numbers.len()-2 {
        current_value += numbers[i+2] - numbers[i-1];
        sliding.push(current_value);
    }
    part1(&sliding)

}