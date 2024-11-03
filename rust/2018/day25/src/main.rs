use std::collections::HashSet;

#[derive(Debug,Copy,Clone,PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
    t: i32
}

impl Point {
    fn distance(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs() + (self.t - other.t).abs()
    }
}

fn main() {
    let data = std::fs::read_to_string("./input.txt").unwrap();
    let mut points = data.split('\n').map(|l| l.split(',').map(|v| v.parse::<i32>().unwrap())).map(|mut n| Point{x: n.next().unwrap(), y: n.next().unwrap(), z: n.next().unwrap(), t: n.next().unwrap()} ).collect::<Vec<_>>();
    let mut constellation_count = 0;
    while ! points.is_empty() {
        let next_constellation =  pull_constellation(points);
        points = next_constellation.1;
        constellation_count +=1;
    }
    println!("Answer: {}", constellation_count);
}


fn pull_constellation(points: Vec<Point>) -> (HashSet<Point>, Vec<Point>) {
    let mut constellation = HashSet::new();
    let start = points[0];
    let mut unevaluated = vec![start];
    while let Some(next_point) = unevaluated.pop() {
        constellation.insert(next_point);
        for point in &points {
            if ! constellation.contains(point) && next_point.distance(point) <= 3{
                unevaluated.push(*point);
            }
        }
    }

    let remnant = points.iter().filter(|x| !constellation.contains(x)).copied().collect::<Vec<_>>();

    (constellation, remnant)
}