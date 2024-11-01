use std::collections::VecDeque;

fn main() {
    let mut list = VecDeque::from([0]);
    let inc = 348;
    for i in 1..50000001 {
        list.rotate_left(inc % list.len());
        list.push_back(i);
        if i == 2017 {
            println!("Part 1: {:?}", list[0]);
        }
    }
    while list[0] != 0 {
        list.pop_front();
    }
    println!("Part 2: {}", list[1]);
} 
