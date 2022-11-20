use std::fs::read_to_string;

fn main() {
    let data = read_to_string("./input.txt").unwrap();
    let mut boxes = data.split_ascii_whitespace().map(|b| b.parse::<i32>().unwrap()).collect::<Vec<_>>();
    boxes.sort_by(|a, b| b.cmp(a));
    let ways = ways_to_make(150, &boxes);
    let min = ways.iter().map(|v| v.len()).min().unwrap();
    println!("Part 1: {}", ways.len());
    println!("Part 2: {}", ways.iter().filter(|v| v.len() == min).count());

}


fn ways_to_make(target:i32, boxes: &[i32]) -> Vec<Vec<i32>> {
    let mut ways: Vec<Vec<i32>> = Vec::new();
    for (i,n) in boxes.iter().enumerate() {
        match n {
            n  if n < &target => {
                let mut recurse = ways_to_make(target-n , &boxes[i+1..]);
                for v in recurse.iter_mut() {
                    v.insert(0, *n);
                }
                ways.append(&mut recurse);
            },
            n if n == &target => {
                ways.push(vec![*n]);
            },
            _ => {}
        }
    }
    ways
}