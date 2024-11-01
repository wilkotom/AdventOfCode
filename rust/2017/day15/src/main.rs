fn main() {
    let mut a = 783;
    let mut b = 325;
    let mask = 2_i64.pow(16) -1;
    let mut count = 0;
    for _ in 0..40_000_000 {
        a = (a * 16807) % 2147483647;
        b = (b * 48271) % 2147483647;
        // println!("{} {}", a, b);
        if a & mask == b & mask {
            count +=1;
        }

    }
    println!("Part 1: {}", count);
    a = 783;
    b = 325;
    count = 0;
    for _ in 0..5_000_000 {
        a = (a * 16807) % 2147483647;
        while a % 4 != 0 {
            a = (a * 16807) % 2147483647;
        }
        b = (b * 48271) % 2147483647;
        while b % 8 != 0 {
            b = (b * 48271) % 2147483647;
        }
        // println!("{} {}", a, b);
        if a & mask == b & mask {
            count +=1;
        }

    }
    println!("Part 2: {}", count);

}
