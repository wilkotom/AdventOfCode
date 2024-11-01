use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

#[derive(Debug,Clone,Copy,Hash, Eq, PartialEq)]
struct Coordinate {
    x: i32,
    y: i32
}


fn main() {
    let coords = std::fs::read_to_string("./input.txt")
                        .unwrap()
                        .split('\n')
                        .map(|l| l.split(", "))
                        .map(|mut s| Coordinate {x: s.next().unwrap().parse::<i32>().unwrap(),
                                                              y: s.last().unwrap().parse::<i32>().unwrap()})
                        .collect::<Vec<_>>();

    let mut area_map: HashMap<Coordinate, HashMap<i32, Vec<Coordinate>>> = HashMap::new();
    let min_x = coords.iter().map(|c| c.x).min().unwrap() -1;
    let max_x = coords.iter().map(|c| c.x).max().unwrap() +1;
    let min_y = coords.iter().map(|c| c.y).min().unwrap() -1;
    let max_y = coords.iter().map(|c| c.y).max().unwrap() +1;

    let mut part2_answer = 0;

    for x in min_x..= max_x {
        for y in min_y ..= max_y {
            let mut all_distances = 0;
            for coord in coords.iter() {
                let distance = (x - coord.x).abs() + (y - coord.y).abs(); 
                area_map.entry(Coordinate{x, y})
                    .or_insert(HashMap::new())
                    .entry(distance)
                    .or_insert(vec![])
                    .push(*coord);
                all_distances += distance;
            }
            if all_distances > 10000 {
                part2_answer +=1;
            }
        }
    }
    let mut infinite = HashSet::new();
    let mut square_radii = HashMap::new();
    for (point, distances) in area_map{
        let points = distances.get(&distances.keys().sorted().next().unwrap()).unwrap();
        if point.x == min_x || point.x == max_x || point.y == min_y || point.y == max_y && points.len() == 1 {
            infinite.insert(points.iter().next().unwrap().clone());
        } else if points.len() ==1 {
            let target = points.iter().next().unwrap();
            square_radii.entry(*target).or_insert(0);
            *square_radii.get_mut(&target).unwrap() += 1;
        }
    }
    
    for coord in infinite {
        square_radii.remove(&coord);
    }
    println!("Part 1: {}", square_radii.values().max().unwrap());
    println!("Part 2: {}", part2_answer);


}
