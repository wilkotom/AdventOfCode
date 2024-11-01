fn main() {

    let input_lengths = std::fs::read_to_string("./input.txt").unwrap();
    part1(&input_lengths);
    part2(&input_lengths);
}

fn part1(length_sequence: &str) {
    let mut list = (0..256).collect::<Vec<_>>();
    let list_len = list.len();
    let mut position = 0;
    
    for (skip_size, input) in length_sequence.split(',').map(|x| x.parse::<usize>().unwrap()).enumerate() {

        let start = position % list.len();
        let mut new_list: Vec<i32> = Vec::new();
        for i in start .. (start + input) {
            new_list.push(list[i % list.len()]);
        }
        for i in start .. (start + input) {
            list[i % list_len] = new_list.pop().unwrap();
        }
        position = position + skip_size + input;
    }
    println!("Part 1: {:?}", list[0] * list[1]);

}


fn part2(length_sequence: &str) {
    let mut list = (0..256).collect::<Vec<_>>();
    let list_len = list.len();
    let mut position = 0;
    let mut skip_size = 0;

    let mut inputs = length_sequence.chars().map(|c| c as usize).collect::<Vec<_>>();
    inputs.append(&mut vec![17,31,73,47,23]);

    for _ in 0..64 {
        for input in &inputs {

            let start = position % list.len();
            let mut new_list: Vec<i32> = Vec::new();
            for i in start .. (start + input) {
                new_list.push(list[i % list.len()]);
            }
            for i in start .. (start + input) {
                list[i % list_len] = new_list.pop().unwrap();
            }
            position = position + skip_size + input;
            skip_size +=1;
        }
    }
    print!("Part 2: ");
    for i in 0..16 {
        
        let mut running = 0;
        for j in 0..16 {
            running ^= list[i*16+j];
        }
        print!("{:02x}", running);
    }
    println!();

}