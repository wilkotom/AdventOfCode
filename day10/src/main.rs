use std::collections::HashSet;

#[derive(Debug,Clone,Copy, Hash, PartialEq, Eq)]
struct Coordinate {
    x: i32,
    y: i32
}

#[derive(Debug,Clone,Copy)]
struct Star {
    position: Coordinate,
    velocity: Coordinate
}

impl Star {
    fn advance(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
    }

    fn rewind(&mut self) {
        self.position.x -= self.velocity.x;
        self.position.y -= self.velocity.y;
    }
}

fn main() {
    let mut stars = vec![];
    for line in std::fs::read_to_string("./input.txt").unwrap().split('\n') {
        let mut fields = line.split('>');
        let position = &fields.next().unwrap()[10..].split(',').map(|x| x.trim().parse::<i32>().unwrap()).collect::<Vec<_>>();
        let velocity = &fields.next().unwrap()[11..].split(',').map(|x| x.trim().parse::<i32>().unwrap()).collect::<Vec<_>>();
        stars.push(Star{ position: Coordinate { x: position[0], y: position[1]},
                         velocity: Coordinate { x: velocity[0], y: velocity[1]}});
    }
    let mut field_width = i32::MAX;
    let mut next_width = field_width -1;
    let mut step_count = 0;
    while next_width < field_width {
        field_width = next_width;
        for star in stars.iter_mut() {
            star.advance();
        }
        let min_x = stars.iter().map(|s| s.position.x).min().unwrap();
        let max_x = stars.iter().map(|s| s.position.x).max().unwrap();
        next_width = max_x - min_x;
        step_count += 1;
    }

    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    let mut positions = HashSet::new();
    for star in stars.iter_mut() {
        star.rewind();
        min_x = min_x.min(star.position.x);
        max_x = max_x.max(star.position.x);
        min_y = min_y.min(star.position.y);
        max_y = max_y.max(star.position.y);
        positions.insert(star.position);
    }
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if positions.contains(&Coordinate{x,y}) {
                print!("#");
            } else{
                print!(" ");
            }
        }
        println!();
    }

    println!("Part 2: {}", step_count -1);
}
