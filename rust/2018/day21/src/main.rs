use std::collections::HashSet;

fn main() {
run_program();
}


fn run_program(){

    let mut seen = HashSet::new();
    let mut last= 0;
    
    let mut r3: i64;
    
    let mut r5 = 0;
    loop {
        r3 = r5 | 65536;
        r5 = 9010242;
        loop {
            r5 = (((r5 + (r3 & 255)) & 16777215) * 65899) & 16777215;
            if r3 < 256 {
                if seen.contains(&r5) {
                    println!("Part 2: {}", last);
                    return;
                }
                if last == 0 {
                    println!("Part 1: {}", r5);
                }
                seen.insert(r5);
                last = r5;
                break;
            } else {
                r3 /= 256;
            }
        }
    }
}