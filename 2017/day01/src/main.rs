fn main() {
    let data = std::fs::read_to_string("./input.txt").unwrap().chars().collect::<Vec<_>>();


    let mut total = 0;
        let length = data.len();
    for i in 0..data.len() {
        if data[i] == data[(i + 1) % length] {
            total += data[i].to_digit(10).unwrap();
        }
    }
    println!("{}", total);
    total = 0;
    for i in 0..data.len() {
        println!("{} {} {} {}", i, (i + (length /2)) % length, data[i], data[(i + (length /2)) % length]) ;
        if data[i] == data[(i + (length /2)) % length] {
            total += data[i].to_digit(10).unwrap();
        }
    }
    println!("{}", total);

}
