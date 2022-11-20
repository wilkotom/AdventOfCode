fn main() {
    let col = 3019;
    let row = 3010;
    let n = (row + col ) -1;
    let iter_count = (n*n + n) /2 - (n - col);
    let mut start: isize = 20151125;
    for _ in 0..iter_count -1 {
        start = (start * 252533) % 33554393;
    }
    println!("{}", start);
}
 