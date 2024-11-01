use hashbrown::HashMap;

fn main() {
    let data = std::fs::read_to_string("./input.txt").unwrap();
    let mut registers: HashMap<&str, isize> = HashMap::new();
    let mut part2_high =0;
    for line in data.split('\n') {
        let mut tokens = line.split_ascii_whitespace();
        let destination = tokens.next().unwrap();
        let oper = tokens.next().unwrap();
        let increment = tokens.next().unwrap().parse::<isize>().unwrap();
        tokens.next();
        let left_number = *registers.get(tokens.next().unwrap()).unwrap_or(&0);
        let cmp = tokens.next().unwrap();
        let right_number = tokens.next().unwrap().parse::<isize>().unwrap();
        
        if match cmp {
            ">" => {
                left_number > right_number
            },
            ">=" => {
                left_number >= right_number
            }
            "<" => {
                left_number < right_number
            },
            "<=" => {
                left_number <= right_number
            }
            "!=" => {
                left_number != right_number
            },
            "==" => {
                left_number == right_number
            },
            _ => unimplemented!()
        } {

            let reg_val = *registers.get(destination).unwrap_or(&0);
            let new_val = match oper {
                "inc" => {reg_val + increment},
                "dec" => {reg_val - increment},
                _ => unimplemented!()
            };
            part2_high = part2_high.max(new_val);
            registers.insert(destination, new_val);
        }
    }

    println!("Part 1: {:?}", registers.values().max().unwrap());
    println!("Part 2: {:?}", part2_high);

}
