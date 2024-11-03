use std::{fmt::Display, ops::{Add, Sub}};
use hashbrown::{HashSet, HashMap};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
    z: i32
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

impl Add for Coordinate{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Sub for Coordinate{
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl Coordinate {
    fn rotate(&self, rotation: i32) -> Coordinate {
        let x = self.x;
        let y = self.y;
        let z = self.z;
        match rotation {
          0  => Coordinate{ x,    y,    z   },
          1  => Coordinate{ x: y, y:-x, z},
          2  => Coordinate{ x:-x, y:-y, z},
          3  => Coordinate{ x:-y, y: x, z},
          4  => Coordinate{ x: z, y, z:-x},
          5  => Coordinate{ x: y, y:-z, z:-x},
          6  => Coordinate{ x: -z,y:-y, z:-x},
          7  => Coordinate{ x:-y, y: z, z:-x},
          8  => Coordinate{ x: z, y:-x, z:-y},
          9  => Coordinate{ x:-x, y:-z, z:-y},
          10 => Coordinate{ x:-z, y: x, z:-y},
          11 => Coordinate{ x, y: z, z:-y},
          12 => Coordinate{ x: z, y:-y, z: x},
          13 => Coordinate{ x:-y, y:-z, z: x},
          14 => Coordinate{ x:-z, y, z: x},
          15 => Coordinate{ x: y, y: z, z: x},
          16 => Coordinate{ x: z, y: x, z: y},
          17 => Coordinate{ x, y:-z, z: y},
          18 => Coordinate{ x:-z, y:-x, z: y},
          19 => Coordinate{ x:-x, y: z, z: y},
          20 => Coordinate{ x:-x, y, z:-z},
          21 => Coordinate{ x: y, y: x, z:-z},
          22 => Coordinate{ x, y:-y, z:-z},
          23 => Coordinate{ x:-y, y:-x, z:-z},
          _ => unreachable!()
        }
      }
}

#[derive(Debug, Clone)]
struct Scanner {
    location: Option<Coordinate>,
    beacons: Vec<Coordinate>,
    orientation: i32,
}

impl Scanner {
    fn oriented_beacons(&self) -> Vec<Coordinate>{
        let mut output: Vec<Coordinate> = Vec::new();
        for coord in &self.beacons {
            output.push(coord.rotate(self.orientation));
        }
    output
    }
}

fn main() {
    let data = std::fs::read_to_string("./input.txt").unwrap();
    let mut scanners: HashMap<i32, Scanner> = HashMap::new();
    for scanner in data.split("\n\n") {
        let mut lines = scanner.split('\n');
        let mut beacons: Vec<Coordinate> = Vec::new();
        let id = lines.next().unwrap().split_ascii_whitespace().collect::<Vec<_>>().get(2).unwrap().parse::<i32>().unwrap();
        for line in lines {
            let mut numbers = line.split(',');
            let x = numbers.next().unwrap().parse::<i32>().unwrap();
            let y = numbers.next().unwrap().parse::<i32>().unwrap();
            let z = numbers.next().unwrap().parse::<i32>().unwrap();
            beacons.push(Coordinate {x,y,z});
        }
        let location = if id == 0 { Some(Coordinate{x:0, y:0, z:0}) } else {None};
        let orientation = 0;
        scanners.insert(id, Scanner{location, beacons, orientation});
    }    

    let scanner_ids = scanners.keys().copied().collect::<Vec<_>>();
    while !all_aligned(&scanners) {
        'outer: for scanner_id in scanner_ids.clone() {
            let scanner = &scanners[&scanner_id];
            if let Some(scanner_location) = scanner.location {
                let beacons = scanner.oriented_beacons();
                for starting_point in scanner.oriented_beacons() {
                    let fixed_constellation = constellation(&starting_point, &beacons);
                    for candidate_id in scanner_ids.clone() {
                        let candidate = scanners.get_mut(&candidate_id).unwrap();
                        if candidate.location.is_some() {
                            continue;
                        }
                        for target_starting_point in candidate.oriented_beacons() {
                            let target_constellation = constellation(&target_starting_point, &candidate.oriented_beacons());
                            let intersection = fixed_constellation.intersection(&target_constellation).count(); 
                            if intersection == 12 {
                                let start_relative = target_starting_point;
                                let target_location = scanner_location + (starting_point - start_relative);
                                candidate.location = Some(target_location);
                                break 'outer
                            }
                        }
                    }
                }
            }
        }
        for scanner_id in scanner_ids.clone() {
            let scanner = scanners.get_mut(&scanner_id).unwrap();
            if scanner.location.is_none() {
                scanner.orientation = (scanner.orientation +1) % 24 ;
            }
        }
    }

    let mut canonical_beacons: HashSet<Coordinate> = HashSet::new();
    let mut scanner_locations: Vec<Coordinate> = Vec::new();
    for scanner in scanners.values() {
        for beacon in scanner.oriented_beacons() {
            if let Some(location) = scanner.location {
                canonical_beacons.insert(beacon + location);
                scanner_locations.push(location);
            }
        }
    }
    println!("Part 1: {:?}", canonical_beacons.len());
    let mut max_dist = 0;
    for start in &scanner_locations {
        for end in &scanner_locations {
            let x = (start.x - end.x).abs();
            let y = (start.y - end.y).abs();
            let z = (start.z - end.z).abs();
            max_dist = max_dist.max(x+y+z);
        }
    }

    println!("Max distance: {}", max_dist);
}

fn all_aligned(scanners: &HashMap<i32, Scanner>) -> bool {
    for scanner in scanners.values() {
        if scanner.location.is_none() {
            return false;
        }
    }
    true
}

fn constellation(start: &Coordinate, beacons: &[Coordinate]) -> HashSet<Coordinate> {
    let mut results = HashSet::new();
    for beacon in beacons {
        results.insert(Coordinate{x:beacon.x - start.x, y: beacon.y - start.y, z: beacon.z - start.z});
    }
    results

}

