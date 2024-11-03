fn main() {
    let data = std::fs::read_to_string(String::from("./input.txt")).unwrap_or(String::from(""));
    let lines = data.split("\n").collect::<Vec<_>>();
    let mut uncompressed_length_part1 = 0;
    let mut uncompressed_length_part2 = 0;
    for line in lines {
        uncompressed_length_part1 += decompress_part1(String::from(line)).len();
        uncompressed_length_part2 += decompress_part2(&String::from(line));
    }
    println!("Total length: {}", uncompressed_length_part1);
    println!("Part 2 length: {}", uncompressed_length_part2);

}

fn decompress_part1(line: String) -> String {
    let mut c= 0;
    let chars = line.chars().collect::<Vec<_>>();
    let mut output = String::new();
    while c < line.len(){
        match chars.get(c) {
            Some('(') => {
                let mut expr = String::new();
                c+=1;
                while chars.get(c).unwrap() != &')'{
                    expr.push(*chars.get(c).unwrap());
                    c +=1;
                }
                c+=1;
                let numbers = expr.split("x").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<_>>();
                let repetitions = numbers[1];
                let mut length = numbers[0];
                let mut repeated = String::new();
                while length > 0 {
                    repeated.push(*chars.get(c).unwrap());
                    length -= 1;
                    c += 1;
                }
                for _ in 0..repetitions {
                    output += &repeated;
                }

            }
            Some(x) => {
                output.push(*x);
                c += 1;
            },
            None => {}
        }
    }

    output
}

fn decompress_part2(line: &str) -> i64 {
    let mut c= 0;
    let chars = line.chars().collect::<Vec<_>>();
    let mut score = 0;
    while c < line.len(){
        match chars.get(c) {
            Some('(') => {
                let mut expr = String::new();
                c+=1;
                while chars.get(c).unwrap() != &')'{
                    expr.push(*chars.get(c).unwrap());
                    c +=1;
                }
                c+=1;
                let numbers = expr.split("x").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
                let repetitions = numbers[1];
                let mut length = numbers[0];
                let mut repeated = String::new();
                while length > 0 {
                    repeated.push(*chars.get(c).unwrap());
                    length -= 1;
                    c += 1;
                }
                score += decompress_part2(&repeated) * repetitions;

            }
            Some(_) => {
                score +=1;
                c += 1;
            },
            None => {}
        }
    }
    score
}