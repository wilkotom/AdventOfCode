use std::{fs::read_to_string, collections::{HashMap,HashSet}, f64::consts::PI};

fn main() {
    let field = read_to_string("./day10/input.txt").unwrap();
    let field = field_to_map(&field);
    let home_asteroid = part1(&field);
    part2(home_asteroid, &field);
}


fn part1(field: &HashSet<(isize,isize)> ) -> (isize, isize){
    let mut highest = 0;
    let mut best = (-1, -1);
    for (x,y) in field {
        let score = visible_others(*x, *y, field);
        if score.len() > highest {
            highest = score.len();
            best = (*x,*y);
        }
    }
    println!("Part 1: {} {:?}", highest, best);
    best
}

fn part2(home: (isize, isize), field: &HashSet<(isize,isize)> ) {
    let angle_asteroids = visible_others(home.0, home.1, field);
    let mut angles = angle_asteroids.keys().collect::<Vec<_>>();
    angles.sort();
    // Not guaranteed to work for all inputs; there are no asteroids lined up with the 200th bearing 
    // that the laster points to. If there were, we would need to sort the list of asteroids on that 
    // bearing and select the first.
    // Additionally I'm assuming that there are n> 200 asteroids visible from the target location.
    let result = (angle_asteroids[angles[199]][0].0 * 100) + angle_asteroids[angles[199]][0].1;
    println!("Part 2: {:?}", result);
}

fn field_to_map(field: &str) -> HashSet<(isize,isize)> {
    let mut map = HashSet::new();   

    for (row, line) in field.split('\n').enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                map.insert((col as isize, row as isize));
            }
        }
    }

    map
}

fn visible_others(x: isize, y: isize, field: &HashSet<(isize,isize)>) -> HashMap<i64,Vec<(isize,isize)>> {

    let mut results = HashMap::new();

    for  (asteroid_x, asteroid_y) in field {
        if asteroid_x == &x && asteroid_y == &y {
            continue;
        }
        // Angle between y-axis and bearing to asteroid, in hundredths of a degree 
        // Rust doesn't allow hashing of floating point numbers, and working in large multiples of pi radians is 
        // less easy on the mental arithmetic.
        let angle = (36000 - ((((x - asteroid_x) as f64 ).atan2((y - asteroid_y) as f64) / PI) * 18000_f64) as i64) % 36000;
        results.entry(angle as i64).or_insert_with(Vec::new);
        results.get_mut(&(angle as i64)).unwrap().push((*asteroid_x, *asteroid_y));
    }

    results

}

#[test]
fn test_north() {
    let field = ".#\n.#";
    assert_eq!(Some(&vec![(1,0)]), visible_others(1,1, &field_to_map(field)).get(&0));
}

#[test]
fn test_south() {
    let field = ".#\n.#";
    assert_eq!(Some(&vec![(1,1)]), visible_others(1,0, &field_to_map(field)).get(&18000));
}

#[test]
fn test_east() {
    let field = "##";
    assert_eq!(Some(&vec![(1,0)]), visible_others(0,0, &field_to_map(field)).get(&9000));
}

#[test]
fn test_west() {
    let field = "##";
    assert_eq!(Some(&vec![(0,0)]), visible_others(1,0, &field_to_map(field)).get(&27000));
}