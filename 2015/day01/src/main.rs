fn main() {
    let data = std::fs::read_to_string("./input.txt").unwrap();
    let mut floor = 0;
    for (i, c) in data.chars().enumerate() {
        match c {
            '(' => {floor +=1}
            ')' => {floor -=1}
            _ => unimplemented!()
        }
        if floor == -1 {
            println!("{}", i +1);
        }
    }
    println!("{}", floor);
}
