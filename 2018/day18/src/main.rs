use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Coordinate {
    x: i32,
    y: i32
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Acre {
    Open,
    Trees,
    LumberYard,
    OutOfBounds
}

type LumberCollectionArea = HashMap<Coordinate, Acre>;


trait LumberCollectionAreaMethods {
    fn display(&self) -> String;
}

impl LumberCollectionAreaMethods for LumberCollectionArea {
    fn display(&self) -> String{
        let mut result = String::new();
        for y in 0..self.keys().map(|n| n.y).max().unwrap() +1{
            for x in 0..self.keys().map(|n| n.x).max().unwrap() +1 {
                result.push(match self.get(&Coordinate{x,y}) {
                    Some(Acre::Open) => '.',
                    Some(Acre::Trees) => '|',
                    Some(Acre::LumberYard) => '#',
                    _ => unimplemented!()
                });
            }
            result.push('\n');
        }
        result
    }
}

impl Coordinate {
    fn get_neighbours(&self) -> Vec<Coordinate> {
        vec![
            Coordinate{x: self.x-1, y: self.y-1},
            Coordinate{x: self.x,   y: self.y-1 },
            Coordinate{x: self.x+1, y: self.y-1},

            Coordinate{x: self.x-1, y: self.y},
            Coordinate{x: self.x+1, y: self.y},
            
            Coordinate{x: self.x-1, y: self.y+1},
            Coordinate{x: self.x,   y: self.y+1},
            Coordinate{x: self.x+1, y: self.y+1},
        ]
    }
}



fn main() {
    let data = std::fs::read_to_string("./input.txt").unwrap();
    let mut map: LumberCollectionArea = HashMap::new();
    for (y, line) in data.split('\n').enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert(Coordinate{x: x as i32,y: y as i32}, match c {
                '.' => Acre::Open,
                '|' => Acre::Trees,
                '#' => Acre::LumberYard,
                _ => unimplemented!()
            });
        }
    }

    let mut seen: HashMap<String, usize> = HashMap::new();
    let mut counter = 0;
    loop {
        map = generation(map);
        counter +=1;
        if seen.contains_key(&map.display()) {
            break;
        }
        if counter == 10 {
            println!("Part 1 answer: {}", map.values().filter(|v| v == &&Acre::Trees).count() * map.values().filter(|v| v == &&Acre::LumberYard).count());
        }
        seen.insert(map.display(), counter);

    }
    let prev_state = seen.get(&map.display()).unwrap();
    let cycle = counter - seen.get(&map.display()).unwrap();

    let duration = (1000000000 - prev_state) % cycle;

    for _ in 0..duration {
        map = generation(map);
    }
    println!("Part 2 answer: {}", map.values().filter(|v| v == &&Acre::Trees).count() * map.values().filter(|v| v == &&Acre::LumberYard).count());

}

fn generation(area: LumberCollectionArea) -> LumberCollectionArea {
    let mut next_state = HashMap::new();

    for (location, acre) in area.iter(){
        next_state.insert(*location, match acre {
            Acre::Open => {
                if location.get_neighbours().iter().filter(|l| area.get(l).unwrap_or(&Acre::OutOfBounds) == &Acre::Trees).count() >=3 { Acre::Trees} else {Acre::Open}
            },
            Acre::Trees => {
                if location.get_neighbours().iter().filter(|l| area.get(l).unwrap_or(&Acre::OutOfBounds) == &Acre::LumberYard).count() >=3 { Acre::LumberYard} else {Acre::Trees}
            }
            Acre::LumberYard => {
                if location.get_neighbours().iter().filter(|l| area.get(l).unwrap_or(&Acre::OutOfBounds) == &Acre::LumberYard).count() >= 1 && 
                    location.get_neighbours().iter().filter(|l| area.get(l).unwrap_or(&Acre::OutOfBounds)== &Acre::Trees).count() >=1
                { Acre::LumberYard} else {Acre::Open}
            }
            Acre::OutOfBounds => unimplemented!(),
        });
    }

    next_state
}
