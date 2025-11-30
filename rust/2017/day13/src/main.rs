fn main() {
    let data = std::fs::read_to_string("./input.txt").unwrap();
    let mut offset = 0;
    let mut caught = true;
    'outer: while caught {
        caught = false;
        let mut severity: i32 = 0;
        for line in data.split('\n') {
            let mut info = line.split(": ");
            let pos = info.next().unwrap().parse::<i32>().unwrap();
            let period = info.next().unwrap().parse::<i32>().unwrap() * 2 -2;
            // println!("{} {}", pos, period);
            if (pos + offset) %period == 0 {
                severity += pos * (period+2) / 2;
                offset += 1;
                caught = true;
                continue 'outer;
                
            }
        }
        println!("Severity Level: {}", severity);
        
    }
    println!("Offset: {}", offset);
}
