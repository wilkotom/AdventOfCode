use std::{collections::HashMap, fs::read_to_string};

use intcode::intcode::start_machine;
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
struct Coordinate {
    x: i64,
    y:i64
}

impl Coordinate {
    fn neighbours(&self) -> Vec<Coordinate> {
        vec![ Coordinate{x: self.x -1, y: self.y},
        Coordinate{x: self.x +1, y: self.y},
        Coordinate{x: self.x, y: self.y -1},
        Coordinate{x: self.x, y: self.y +1},
        ]
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
enum Square {
    Wall,
    Corridor,
    OxygenGenerator}

fn main() {
    let program = read_to_string("./day15/input.txt").unwrap().split(',').map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
    let corridor_map = create_map(program);
    let distances = generate_distances(&corridor_map);
    println!("Part 1: {}", distances.get(&Coordinate{x:0,y:0}).unwrap());
    println!("Part 2: {}", distances.values().max().unwrap());

}

fn generate_distances(corridor_map: &HashMap<Coordinate, Square>) -> HashMap<Coordinate, i32>{
    let (starting_point, _) = corridor_map.iter().find(|(_, s)| *s == &Square::OxygenGenerator).unwrap();
    println!("Starting Point: {:?}", starting_point);
    let mut unevaluated = Vec::new();
    let mut distances = HashMap::new();
    unevaluated.push((0, *starting_point));
    while ! unevaluated.is_empty() {
        let (distance, coordinate) = unevaluated.pop().unwrap();
        if corridor_map.get(&coordinate).unwrap() != &Square::Wall && distances.get(&coordinate).unwrap_or(&i32::MAX) > &distance {
            distances.insert(coordinate, distance);
            for neighbour in coordinate.neighbours() {
                unevaluated.push((distance +1, neighbour));
            }
        }
    }
    distances
}

fn create_map(program: Vec<i64>) -> HashMap<Coordinate, Square> {
    let (input, output) = start_machine(&program);
    let mut corridor_map = HashMap::new();
    let mut facing = 1;
    let mut location = Coordinate{x:0, y:0};
    corridor_map.insert(location, Square::Corridor);

    // Loop while there are corridor squares which aren't surrounded by other squares
    'outer: while corridor_map.iter()
            .filter(|(_, i)| *i != &Square::Wall)
            .any(|(i, _)| 
                i.neighbours().iter().filter(|k| corridor_map.contains_key(k)).count() != 4) {

        let directions = match facing {
            1 => [3,1,4,2],  // West / North / East / South
            2 => [4,2,3,1],  // East / South / West  North
            3 => [2,3,1,4],  // South / West / North / East
            4 => [1,4,2,3],  // North / East / South / West
            _ => unimplemented!()
        };

        for direction in directions {
            facing = direction;
            input.send(direction).unwrap();
            match output.recv() {
                Ok(0) => { 
                    let mut wall = location;
                    match facing {
                        1 => wall.y += 1,
                        2 => wall.y -= 1,
                        3 => wall.x -= 1,
                        4 => wall.x += 1,
                        _ => unimplemented!()
                    }
                    corridor_map.insert(wall,Square::Wall);
                }

                Ok(n) => {
                    match facing {
                        1 => location.y += 1,
                        2 => location.y -= 1,
                        3 => location.x -= 1,
                        4 => location.x += 1,
                        _ => unimplemented!()
                    }
                    corridor_map.insert(location, if n == 1 {Square::Corridor} else {Square::OxygenGenerator});
                    break;
                },
                Err(_) => {break 'outer;},
            }
        }

    }
    corridor_map
}