use std::collections::HashMap;

fn main() {
    let data = std::fs::read_to_string("./input.txt").unwrap();
    let mut banks = data.split_ascii_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let len = banks.len();
    let mut seen: HashMap<Vec<usize>, i32> = HashMap::new();
    let mut cycles = 0;
    while ! seen.contains_key(&banks) {

        seen.insert(banks.clone(), cycles);
        let max = *banks.iter().max().unwrap() as usize;
        let mut index = 0;
        while banks[index] != max {
            index += 1;
        }
        banks[index] = 0;
        for i in index+1 .. index+1+ banks.len() { 
            banks[i % len] += max / banks.len();
        }
        let remainder = max % banks.len();
        for i in index+1 .. index+1+remainder {
            banks[i % len] +=1;
        }
        cycles += 1;
    }

    println!("{}", cycles - seen[&banks]);
}
