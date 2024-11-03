use std::collections::HashSet;

fn main() {
    let raw_data = std::fs::read_to_string("./input.txt").unwrap();
    let data = raw_data.split('\n')
    .clone()
    .collect::<Vec<&str>>();
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));

}


fn part1(data: &Vec<&str>) -> u64 {
    let mut twos = 0;
    let mut threes = 0;
    for item in data {
        let mut counts = HashSet::new();
        for c in 'a'..='z' {
            counts.insert(item.matches(c).count());
        }
        if counts.contains(&2) {
            twos +=1;
        }
        if counts.contains(&3) {
            threes +=1;
        }
    }

    twos * threes
}

fn part2(data: &[&str]) -> String {
    // O(n^2 w - ugh)
    for l in 0..data.len() {
        for r in l+1..data.len() {
            for n in 0..data[0].len() {
                if data[l][..n] == data[r][..n] && data[l][n+1..] == data[r][n+1..] {
                    return format!("{}{} ", &data[l][..n], &data[l][n+1..]);
                } 
            }
        }
    }
    String::new()
}