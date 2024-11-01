fn main() {
    let mut lanternfish = vec![0_usize; 9];
    for fish in std::fs::read_to_string("./input.txt").unwrap().split(',').map(|x| x.parse::<usize>().unwrap()){
        lanternfish[fish] += 1;
    }
    for _ in 0..256 {
        lanternfish.rotate_left(1);
        lanternfish[6] += lanternfish[8];
    }
    println!("{}", lanternfish.iter().sum::<usize>())
}