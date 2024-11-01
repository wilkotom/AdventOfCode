fn main() {
    let data = std::fs::read_to_string("./input.txt").unwrap();
    let mut pairs: Vec<(i32,i32)> = Vec::new();
    for line in data.split('\n') {
        let mut numbers = line.split('/');
        pairs.push((numbers.next().unwrap().parse::<i32>().unwrap(), numbers.next().unwrap().parse::<i32>().unwrap()));
    }

    // println!("{:?}", make_bridges(0, pairs.clone()));
    let mut part1_answer = 0;
    let mut part2_answer =0;
    let mut max_len = 0;
    for bridge in make_bridges(0, pairs) {
        part1_answer =  part1_answer.max(bridge.iter().map(|x| x.0 + x.1).sum::<i32>());
        max_len = max_len.max(bridge.len());
        if bridge.len() == max_len {
            part2_answer = part2_answer.max(bridge.iter().map(|x| x.0 + x.1).sum::<i32>());
        }
        
    }
    println!("Part 1: {}", part1_answer);
    println!("Part 2: {}", part2_answer);

}

fn make_bridges(starting_point: i32, blocks: Vec<(i32,i32)>) -> Vec<Vec<(i32,i32)>> {

    let mut result = Vec::new();
    for block in blocks.iter() {
        if block.0 == starting_point || block.1 == starting_point {
            result.push(vec![*block]);
            let next_blocks = blocks.iter().filter(|b| *b != block).copied().collect::<Vec<_>>();
            let next_start = if block.0 == starting_point { block.1 } else {block.0};
            for mut next_bridge in make_bridges(next_start, next_blocks) {
                next_bridge.insert(0, *block);
                result.push(next_bridge);
            }

        }
    }

    result
}
