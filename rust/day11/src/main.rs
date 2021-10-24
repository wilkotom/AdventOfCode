use std::fs;
use std::io::Error;
use std::collections::HashMap;

#[derive(Copy,Debug,PartialEq)]
enum SeatPosition {
    Empty,
    Occupied,
    Floor,
    OutOfBounds
}

impl Clone for SeatPosition{
    fn clone(&self) -> SeatPosition {
        match self {
            SeatPosition::Empty => SeatPosition::Empty,
            SeatPosition::Occupied => SeatPosition::Occupied,
            SeatPosition::Floor => SeatPosition::Floor,
            SeatPosition::OutOfBounds => SeatPosition::OutOfBounds
        }
    }
}

#[derive(Copy,Clone,PartialEq,Eq,Hash,Debug)]
struct Coordinate {
    x: i64,
    y: i64
}

fn main() -> Result<(), Error> {
    let seatmap = read_seatplan("./input.txt".to_string()).unwrap();
    println!("{}", run_sim(seatmap.clone(), part_1_neighbour_map, 4));
    println!("{:?}", run_sim(seatmap.clone(), part_2_neighbour_map, 5));
    Ok(())
}

fn run_sim(mut seatplan: HashMap<Coordinate, SeatPosition>, 
         neighbour_map: fn(seatplan: &HashMap<Coordinate, SeatPosition>) -> HashMap<Coordinate, Vec<Coordinate>>,
         threshold: i64
        ) -> i64 {
    let neighbour_map = neighbour_map(&seatplan);
    let mut modified = true;
    while modified {

        let result = generation(seatplan, &neighbour_map, threshold);
        modified = result.1;
        seatplan = result.0;
    }
    seatplan.iter().filter(|&(_, v)| v == &SeatPosition::Occupied).count() as i64
    
}


fn generation(seatplan: HashMap<Coordinate, SeatPosition>, 
              neighbours: &HashMap<Coordinate, Vec<Coordinate>>, 
              threshold: i64) -> (HashMap<Coordinate, SeatPosition>, bool) {
    let mut result: HashMap<Coordinate, SeatPosition> = HashMap::new();
    let mut modified = false;
    for seat in seatplan.keys() {
         result.insert(*seat, match (get_neighbour_count(&seatplan, neighbours.get(&seat.clone()).unwrap_or(&(Vec::new()))), seatplan.get(seat).unwrap()) {
            (0, SeatPosition::Empty) => {modified = true;
                SeatPosition::Occupied},
            (x, SeatPosition::Occupied) if x >= threshold => {modified = true; 
                SeatPosition::Empty},
            (_, sp) => *sp
        });
    }
    (result, modified)
}

fn read_seatplan(filename: String) -> Result<HashMap<Coordinate, SeatPosition>, Error> {
    let mut seatmap: HashMap<Coordinate, SeatPosition> = HashMap::new();
    let raw_data = fs::read_to_string(filename)?;
    let lines: Vec<_> = raw_data.split("\n").collect();
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                'L' => {seatmap.insert(Coordinate{x:j as i64, y:i as i64}, SeatPosition::Empty);},
                '.' => {seatmap.insert(Coordinate{x:j as i64, y:i as i64}, SeatPosition::Floor);},
                _ => {}
            }
        } 
    }
    Ok(seatmap)
}

fn get_neighbour_count(seatplan: &HashMap<Coordinate, SeatPosition>, 
                       neighbour_list: &Vec<Coordinate>) -> i64 {
    let mut neighbours: Vec<SeatPosition> = Vec::new();
    for neighbour in neighbour_list
    {
        neighbours.push(*seatplan.get(&neighbour).unwrap_or(&SeatPosition::OutOfBounds));
    }
    neighbours.iter().filter(|&&n| n == SeatPosition::Occupied).count() as i64
}

fn part_1_neighbour_map(seatplan: &HashMap<Coordinate, SeatPosition>) -> HashMap<Coordinate, Vec<Coordinate>> {
    let mut mappings: HashMap<Coordinate, Vec<Coordinate>> = HashMap::new();
    for seat in seatplan.keys() {
        let x = seat.x;
        let y = seat.y; 
        for neighbour in [
            Coordinate{x:x,   y:y+1},
            Coordinate{x:x+1, y:y+1}, 
            Coordinate{x:x+1, y:y},
            Coordinate{x:x-1, y: y+1}
            ] {
                match seatplan.get(&neighbour) {
                    Some(SeatPosition::Floor) | None => {},
                    _  =>  { 
                        mappings.entry(*seat).or_insert(Vec::new()).push(neighbour); 
                        mappings.entry(neighbour).or_insert(Vec::new()).push(*seat);}
                }
        }   
            
    }
    mappings
}

fn part_2_neighbour_map(seatplan: &HashMap<Coordinate, SeatPosition>) -> HashMap<Coordinate, Vec<Coordinate>> {
    let mut mappings: HashMap<Coordinate, Vec<Coordinate>> = HashMap::new();
    for seat in seatplan.keys() {
        for direction in [(1,0), (1,1),(0,1),(-1,1)] {
            let mut factor = 1;
            while seatplan.get(&(Coordinate{x: seat.x + (direction.0 * factor), y: seat.y + (direction.1 * factor)}))
                    .unwrap_or(&SeatPosition::OutOfBounds) == &SeatPosition::Floor {
                factor +=1;
            }
            if seatplan.get(&(Coordinate{x: seat.x + (direction.0 * factor), y: seat.y + (direction.1 * factor)}))
                    .unwrap_or(&SeatPosition::OutOfBounds) == &SeatPosition::Empty {
                mappings.entry(*seat).or_insert(Vec::new()).push(Coordinate{x: seat.x + (direction.0 * factor), y: seat.y + (direction.1 * factor)}); 
                mappings.entry(Coordinate{x: seat.x + (direction.0 * factor), y: seat.y + (direction.1 * factor)}).or_insert(Vec::new()).push(*seat);    
            }
        }   
            
    }
    mappings
}
