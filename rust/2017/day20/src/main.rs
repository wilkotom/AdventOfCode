use std::{ops::{Add, AddAssign}, collections::HashMap};

#[derive(Debug,Clone, Copy, Hash, PartialEq, Eq)]
struct Particle {
    position: Coordinate,
    velocity: Coordinate,
    acceleration: Coordinate,
    id: usize,
    removed: bool
}

#[derive(Debug,Clone, Copy, Hash, PartialEq, Eq)]
struct Coordinate {
    x: i64,
    y: i64,
    z: i64
}

impl Particle {
    fn manhattan_acceleration(&self) -> i64 {
        self.acceleration.x.abs() + self.acceleration.y.abs() + self.acceleration.z.abs()
    }

    fn update(&mut self) {
        self.velocity.x += self.acceleration.x;
        self.velocity.y += self.acceleration.y;
        self.velocity.z += self.acceleration.z;
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self.position.z += self.velocity.z;
    }
    
}


fn main() {
    let mut particles: Vec::<Particle> = Vec::new();
    for (i, line) in std::fs::read_to_string("./input.txt").unwrap().split('\n').enumerate() {
        let mut sections = line.split(">, ");
        let next = sections.next().unwrap();
        let mut xyz = next[3..next.len()].split(',');
        let position = Coordinate{x: xyz.next().unwrap().parse::<i64>().unwrap(), y: xyz.next().unwrap().parse::<i64>().unwrap(), z: xyz.next().unwrap().parse::<i64>().unwrap()};
        let next = sections.next().unwrap();
        let mut xyz = next[3..next.len()].split(',');
        let velocity = Coordinate{x: xyz.next().unwrap().parse::<i64>().unwrap(), y: xyz.next().unwrap().parse::<i64>().unwrap(), z: xyz.next().unwrap().parse::<i64>().unwrap()};
        let next = sections.next().unwrap();
        let mut xyz = next[3..next.len() -1].split(',');
        let acceleration = Coordinate{x: xyz.next().unwrap().parse::<i64>().unwrap(), y: xyz.next().unwrap().parse::<i64>().unwrap(), z: xyz.next().unwrap().parse::<i64>().unwrap()};
        particles.push(Particle{position, velocity, acceleration, id: i, removed: false});

    }

    let mut lowest_acceleration = i64::MAX;
    let mut nearest_particle = 0;
    for (i, p) in particles.iter().enumerate() {
        if p.manhattan_acceleration() < lowest_acceleration {
            nearest_particle = i;
            lowest_acceleration = p.manhattan_acceleration();
        }
    }
    println!("Part 1: {}", nearest_particle);

    for _ in 0..1000 {
        let mut locations: HashMap<Coordinate, usize> = HashMap::new();
        let mut exploding: Vec<usize> = Vec::new();
        for (i, particle) in particles.iter_mut().enumerate().filter(|(_, p)| ! p.removed) {
            particle.update();
            if let std::collections::hash_map::Entry::Vacant(e) = locations.entry(particle.position) {
                e.insert(i);
            } else {
                particle.removed = true;
                exploding.push(locations[&particle.position]);
            }
        }
        for  point in exploding {
            particles[point].removed = true;
        }
    }

    println!("Part 2: {}", particles.iter().filter(|p| !p.removed).count());
}
