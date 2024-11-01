use std::{ops::{Add, AddAssign}, fs::read_to_string};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coordinate {
    x: i32,
    y: i32,
    z: i32
}

impl Add for Coordinate {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl AddAssign for Coordinate {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        };
    }
}


#[derive(Debug, Clone)]
struct Planet  {
	position: Coordinate,
	velocity: Coordinate,
	period: Coordinate,
	start_pos: Coordinate,
	start_velocity: Coordinate
}


fn main() {
    let mut planets = read_planets("./day12/input.txt");
    let mut iterations = 1;
    let mut x_period = 0;
    let mut y_period = 0;
    let mut z_period = 0;

    while x_period==0 || y_period ==0 || z_period == 0 {

        // Apply gravity
        for i in 0..planets.len() {
            for j in i+1..planets.len() {
                match planets[i].position.x.cmp(&planets[j].position.x) {
                    std::cmp::Ordering::Less => { 
                        planets[i].velocity.x += 1;
                        planets[j].velocity.x -= 1; 
                    },
                    std::cmp::Ordering::Equal => {},
                    std::cmp::Ordering::Greater => {
                        planets[i].velocity.x -= 1;
                        planets[j].velocity.x += 1;},
                }

                match planets[i].position.y.cmp(&planets[j].position.y) {
                    std::cmp::Ordering::Less => { 
                        planets[i].velocity.y += 1;
                        planets[j].velocity.y -= 1; 
                    },
                    std::cmp::Ordering::Equal => {},
                    std::cmp::Ordering::Greater => {
                        planets[i].velocity.y -= 1;
                        planets[j].velocity.y += 1;},
                }
                match planets[i].position.z.cmp(&planets[j].position.z) {
                    std::cmp::Ordering::Less => { 
                        planets[i].velocity.z += 1;
                        planets[j].velocity.z -= 1; 
                    },
                    std::cmp::Ordering::Equal => {},
                    std::cmp::Ordering::Greater => {
                        planets[i].velocity.z -= 1;
                        planets[j].velocity.z += 1;},
                }
            }
        }
        // Apply velocity
        for planet in planets.iter_mut() {
            planet.position += planet.velocity;
        }
        
        iterations += 1;


        if iterations > 1 && x_period == 0 && planets.iter().all(|p| p.position.x == p.start_pos.x){
            x_period = iterations;
        }
        
        if iterations > 1 && y_period == 0 && planets.iter().all(|p| p.position.y == p.start_pos.y){
            y_period = iterations;
        }

        if iterations > 1 && z_period == 0 && planets.iter().all(|p| p.position.z == p.start_pos.z){
            z_period = iterations;
        }

        if iterations == 1 {
            for planet in planets.iter_mut() {
                planet.start_velocity = planet.velocity;
                planet.start_pos = planet.position;
            }
        }

        if iterations == 1000 {
            let mut total = 0;

            for planet in planets.clone() {
                total += (planet.position.x.abs() + planet.position.y.abs() + planet.position.z.abs()) * (planet.velocity.x.abs() + planet.velocity.y.abs() + planet.velocity.z.abs());
            }
        
            println!("Part 1: {}", total);
        
        }
    }
    let step_size:i64 = x_period.max(y_period.max(z_period));

    let mut period = step_size;
    while ! ((period % x_period == 0) && (period % y_period == 0) && (period % z_period == 0))  {
        period += step_size;
    }
    println!("Part 2: {:#?}", period);
}


fn read_planets(filename: &str) -> Vec<Planet>{
    let data = read_to_string(filename).unwrap();
    let mut planets = Vec::new();
    let zeroed = Coordinate{x:0, y:0, z:0};
    for line in data.split('\n') {
        let mut fields = line.split(", ");
        let x = fields.next().unwrap()[3..].parse::<i32>().unwrap();
        let y = fields.next().unwrap()[2..].parse::<i32>().unwrap();
        let z = fields.next().unwrap()[2..].strip_suffix('>').unwrap().parse::<i32>().unwrap();
        planets.push(  Planet{ position: Coordinate{x, y, z }, 
            velocity: zeroed,
            period: zeroed,
            start_pos: Coordinate{x, y, z },
            start_velocity: zeroed});
    }
    planets
}