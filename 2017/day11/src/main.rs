



fn main() {
    let data = std::fs::read_to_string("./test.txt").unwrap();
    //let data = "se,sw,se,sw,sw";
    let mut x:i32 = 0;
    let mut y:i32 = 0;
    let mut furthest = 0;
    for instr in data.split(',') {
        match instr {
            "n" => {
                y +=2;
            }
            "ne" => {
                x += 1;
                y += 1;
            }
            "se" => {
                x +=1;
                y -=1;
            }
            "s" => {
                y -=2
            },
            "sw" => {
                x -=1;
                y -=1;
            }
            "nw" => {
                x -=1;
                y +=1;
            }
            _ => unimplemented!()
        }
        furthest = furthest.max(x.abs() + (y.abs() - x.abs()) / 2);
    }
    println!("({},{})", x,y);
    let horizontal = x.abs();
    let vertical = (y.abs() - horizontal) /2;
    println!("Part 1: {} ", horizontal + vertical);
    println!("Part 2: {} ", furthest);
    

}

// 1956 too high