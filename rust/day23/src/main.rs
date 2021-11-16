use std::collections::HashMap;

fn main() {
    let list = "389125467".chars().map(|x| x.to_string().parse::<usize>().unwrap()).collect::<Vec<_>>();
    let mut mapping: HashMap<usize,usize> = HashMap::new();
    for (i, &n) in list.iter().enumerate() {
        let next_i = (i +1) % list.len();
        let cup = list.get(next_i).cloned().unwrap();
        mapping.insert(n, cup);
    }

    shuffle(&mut mapping, list[0], 100);
    part1_result(&1, &mapping);

    let mut mapping: HashMap<usize,usize> = HashMap::new();
    for (i, &n) in list.iter().enumerate() {
        let next_i = (i +1) % list.len();
        let cup = list.get(next_i).cloned().unwrap();
        mapping.insert(n, cup);
    }

    let part_2_cups = 1000000;
    for c in mapping.len()..part_2_cups {
        mapping.insert(c, c+1);
    }
    mapping.insert(part_2_cups, list[0]);
    mapping.insert(list[list.len()-1], list.len()+1);
    shuffle(&mut mapping, list[0], 10000000);
    let first = mapping.get(&1).unwrap();
    let second = mapping.get(&first).unwrap();
    println!("Part 2: {}", first * second);
}

fn shuffle(cups: &mut HashMap<usize,usize>, starting_cup: usize, moves: usize) {

    let mut current_cup = starting_cup;
    for _ in 0..moves {
        let mut next_three: Vec::<usize> = Vec::new();
        let mut next = cups.get(&current_cup).unwrap();
        for _ in 0..3 {
            next_three.push(*next);
            next = cups.get(next).unwrap();
        }
        let mut insertion_point = if current_cup -1 == 0 {cups.len()} else { current_cup -1};
        while next_three.contains(&insertion_point) {
            insertion_point -= 1;
            if insertion_point == 0 {
                insertion_point = cups.len();
            }
        }
        let next_cup = *cups.get(&next_three[2]).unwrap();
        cups.insert(next_three[2],cups[&insertion_point]);
        cups.insert(insertion_point,next_three[0]);
        cups.insert(current_cup, next_cup);
        current_cup = next_cup;

    }

}

fn part1_result(start: &usize, ring: &HashMap<usize,usize>) {
    // print!("({}) ", start);
    print!("Part 1: ");
    let mut next = ring.get(&start).unwrap();
    while next != start {
        print!("{}", next);
        next = ring.get(&next).unwrap();
    }
    println!()
}