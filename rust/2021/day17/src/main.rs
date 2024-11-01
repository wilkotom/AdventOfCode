#[derive(Debug,Copy,Clone)]
struct Target {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64
}
fn main() {
    let data = std::fs::read_to_string("./input.txt").unwrap();
    let mut sections = data.split(", ");
    let mut x_vals = sections.next().unwrap()[15..].split("..");
    let mut y_vals = sections.next().unwrap()[2..].split("..");

    let min_x = x_vals.next().unwrap().parse::<i64>().unwrap();
    let max_x = x_vals.next().unwrap().parse::<i64>().unwrap();
    let min_y = y_vals.next().unwrap().parse::<i64>().unwrap();
    let max_y = y_vals.next().unwrap().parse::<i64>().unwrap();
    
    println!("Target at ({},{}) to ({} {})", min_x, min_y, max_x, max_y);
    let target = Target{min_x, min_y, max_x, max_y};
    let mut best_height = 0;
    let mut counter = 0;
    for x in 0..(max_x+1) {
        for y in min_y..(0 - min_y) {
            let (hit, height) = fire(x,y,target);
            counter += if hit {1} else {0};
            if hit && height > best_height {
                    best_height = height;
            }
        }
    }
    println!("Part 1: {}", best_height);
    println!("Part 2: {}", counter);
}

fn fire(mut x_vel: i64, mut y_vel: i64, target: Target) -> (bool, i64) {
    let mut x = 0;
    let mut y = 0;
    let mut max_y = 0;
    while y >= target.min_y && !(x_vel == 0 && (x < target.min_x || x > target.max_x)) {
        if x >= target.min_x && x <= target.max_x && y >= target.min_y && y <= target.max_y {
            return (true, max_y);
        }
        x += x_vel;
        y += y_vel;
        max_y = (y).max(max_y);
        y_vel -= 1;
        x_vel += x.signum();
    }
    (false, max_y)
}