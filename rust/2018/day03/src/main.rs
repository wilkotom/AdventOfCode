use std::collections::{HashMap, HashSet};

#[derive(Debug,Clone,Copy,Hash,Eq,PartialEq)]
struct Coordinate{
    x: i32,
    y: i32
}
#[derive(Eq,PartialEq, Debug)]
struct FabricClaim {
    id: i32,
    location: Coordinate,
    size: Coordinate
}

fn main() {
    let data = std::fs::read_to_string("./input.txt").unwrap();
    let mut square_overlaps: HashMap<Coordinate,Vec<i32>> = HashMap::new();
    let mut unique_claims: HashSet<i32> = HashSet::new();
    for line in data.split('\n') {
        let claim = generate_claim(line);
        unique_claims.insert(claim.id);
        for x in claim.location.x..(claim.location.x+claim.size.x) {
            for y in claim.location.y..(claim.location.y+claim.size.y) {
                square_overlaps.entry(Coordinate{x,y})
                    .and_modify(|l| l.push(claim.id))
                    .or_insert(vec![claim.id]);
            }
        }
    }
    let mut overlapping_squares = 0;

    for square in square_overlaps.values() {
        if square.len() > 1 {
            overlapping_squares +=1;
            for claim in square {
                unique_claims.remove(claim);
            }
        }
    }
    println!("Part 1: {}", overlapping_squares);
    println!("Part 2: {}", unique_claims.iter().next().unwrap());
}

fn generate_claim(line: &str) -> FabricClaim {
    let fields = line.split(' ').collect::<Vec<_>>();
    let id = fields[0][1..].parse::<i32>().unwrap();
    let mut coords = fields[2][..fields[2].len() -1].split(',');
    let x = coords.next().unwrap().parse::<i32>().unwrap();
    let y = coords.next().unwrap().parse::<i32>().unwrap();

    let mut size = fields[3].split('x');
    let width = size.next().unwrap().parse::<i32>().unwrap();
    let height = size.next().unwrap().parse::<i32>().unwrap();

    FabricClaim{ 
        id, 
        location: Coordinate{ x,y},
        size: Coordinate{x: width, y: height}
    }

}



#[test]
fn test_generate_claim() {
    let raw_claim = "#1 @ 1,3: 4x4";
    assert_eq!(generate_claim(raw_claim), FabricClaim{id: 1, location: Coordinate{x:1, y:3}, size: Coordinate{x: 4, y:4}});

}