
#[derive(Debug)]
struct Disc {
    period: isize,
    start_pos: isize
}

// 88179 is too low!
// 440895 too high
fn main() {
    let data = std::fs::read_to_string("./input.txt").unwrap();
    let mut discs: Vec<Disc> = Vec::new();
    for line in data.split("\n") {
        let words = line.split_ascii_whitespace().collect::<Vec<_>>();
        let start_pos = words[11][0..&words[11].len()-1].parse::<isize>().unwrap() + words[1][1..].parse::<isize>().unwrap();
        let period = words[3].parse::<isize>().unwrap();
        discs.push(Disc{period, start_pos})
    }
    let mut time = 0;
    let mut found = false;
    while !found {
        time +=1;
        found = true;
        for disc in &discs {
            if (time + disc.start_pos) % disc.period != 0 { 
                found = false;
            }
        }
    }
    println!("{}", time);
}


