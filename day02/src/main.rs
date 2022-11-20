fn main() {
    let data = std::fs::read_to_string("./input.txt").unwrap();

    let mut part1_total = 0;
    let mut part2_total = 0;

    for line in data.split('\n') {
        let mut numbers = line.split_ascii_whitespace().map(|x| x.parse::<isize>().unwrap()).collect::<Vec<_>>();
        numbers.sort_unstable();
        part1_total += numbers.last().unwrap() - numbers[0];

        for num in numbers.iter().rev() {
            for div in numbers.iter() {
                if div > num {
                    break;
                } else if num % div == 0 && num != div {
                    // println!("{} {} {} {}", num, div, num%div,  num / div);
                    part2_total += num / div;
                }
            }
        }

    }
    println!("{}", part1_total);
    println!("{}", part2_total);
}
