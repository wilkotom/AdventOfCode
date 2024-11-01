use std::fs::read_to_string;

#[derive(Debug,Copy, Clone, Hash, PartialEq, Eq)]
struct Nanobot {
    x: i64,
    y: i64,
    z: i64,
    radius: i64
}


fn main() {
    let data = read_to_string("./input.txt").unwrap();
    let mut bots = Vec::new();
    for line in data.split('\n') {
        let mut fields = line[5..].split(">, r=");
        let mut  coords = fields.next().unwrap().split(',').map(|v| v.parse::<i64>().unwrap());
        let radius = fields.next().unwrap().parse::<i64>().unwrap();
        let bot = Nanobot{x: coords.next().unwrap(), y: coords.next().unwrap(), z: coords.next().unwrap(), radius};
        bots.push(bot);
    }
    let max_radius = bots.iter().map(|r| r.radius).max().unwrap();
    let centre = bots.iter().find(|b| b.radius == max_radius).unwrap();

    println!("Part 1: {:?}", bots.iter().filter(|b| (b.x - centre.x).abs() + (b.y - centre.y).abs()  + (b.z - centre.z).abs() <= centre.radius).count());


    // Part 2

    // Point with the most overlaps must be within the bounding cuboid between min and max of x, y and Z

    let mut min_corner = Nanobot {
        x: bots.iter().map(|b| b.x).min().unwrap(), 
        y: bots.iter().map(|b| b.y).min().unwrap(), 
        z: bots.iter().map(|b| b.z).min().unwrap(), radius: 0 };
    let mut max_corner = Nanobot {
        x: bots.iter().map(|b| b.x).max().unwrap(), 
        y: bots.iter().map(|b| b.y).max().unwrap(), 
        z: bots.iter().map(|b| b.z).max().unwrap(), radius: 0};

    
    // We should set a search radius of the smallest power of two that's bigger than the box
    let mut search_radius = 1_i64;
    while search_radius < (max_corner.x - min_corner.x) && search_radius < (max_corner.y - min_corner.y) && search_radius < (max_corner.z - min_corner.z) {
        search_radius *= 2;
    }

    let mut best_for_size = 0;
    
    while search_radius > 0 {
        let mut best_count = 0;
        let mut best_location = Nanobot{x:0, y:0, z: 0, radius: 0};
        let mut best_distance = i64::MAX;
        // for each of the points along the edges of the search box such that they're separarated by the search radius,
        // see which point has the most overlaps withing its radius (radis is bigger than cube side /2)
        for x in (min_corner.x .. max_corner.x +1).step_by(search_radius as usize) {
            for y in (min_corner.y .. max_corner.y +1).step_by(search_radius as usize) {
                for z in (min_corner.z .. max_corner.z +1).step_by(search_radius as usize) {
                    let range_bots_count = bots.iter().filter(|b| ((x -b.x).abs() + (y-b.y).abs() + (z-b.z).abs() - b.radius) / search_radius <= 0).count();

                    if range_bots_count > best_count {
                        best_count = range_bots_count;
                        best_distance = x.abs() + y.abs() + z.abs();
                        best_location = Nanobot{x,y,z, radius:0};
                    }
                }
            }
        }
        // re-centre the search on the chosen corner
        best_for_size = best_distance;
        min_corner = Nanobot {
            x: best_location.x - search_radius,
            y: best_location.y - search_radius,
            z: best_location.z - search_radius, radius: 0
        };
        max_corner = Nanobot {
            x: best_location.x + search_radius,
            y: best_location.y + search_radius,
            z: best_location.z + search_radius, radius: 0
        };
        // reduce the search radius
        search_radius /= 2;
    }
    println!("Part 2: {}", best_for_size);

}
