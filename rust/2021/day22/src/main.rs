use std::fs::read_to_string;
use hashbrown::HashSet;

#[derive(Debug,Hash,Eq,PartialEq, Copy, Clone)]
struct Coordinate {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Debug,Hash,Eq,PartialEq, Copy, Clone)]
struct Cuboid {
    top_left_back: Coordinate,
    bottom_right_front:Coordinate
}

impl Cuboid {
    fn intersection(&self, other: &Self) -> Option<Self> {
        if self.bottom_right_front.x < other.top_left_back.x || self.top_left_back.x > other.bottom_right_front.x || 
           self.bottom_right_front.y < other.top_left_back.y || self.top_left_back.y > other.bottom_right_front.y || 
           self.bottom_right_front.z < other.top_left_back.z || self.top_left_back.z > other.bottom_right_front.z {
            None
        } else {
            let top_left_back = Coordinate{
                x: self.top_left_back.x.max(other.top_left_back.x),
                y: self.top_left_back.y.max(other.top_left_back.y),
                z: self.top_left_back.z.max(other.top_left_back.z),
            };
            let bottom_right_front = Coordinate{
                x: self.bottom_right_front.x.min(other.bottom_right_front.x),
                y: self.bottom_right_front.y.min(other.bottom_right_front.y),
                z: self.bottom_right_front.z.min(other.bottom_right_front.z)
            };
            Some(Cuboid{top_left_back, bottom_right_front})
        }
    }


    fn difference(&self, other: &Self) -> Vec<Self> {
        if self.intersection(other).is_some() {
            // println!("Cubes overlap at {:?}", int);
            let tentative: Vec<Cuboid> = vec![

                // bottom layer
                Cuboid{ top_left_back: Coordinate{      x: self.top_left_back.x,       y: self.top_left_back.y,         z: self.top_left_back.z},
                    bottom_right_front: Coordinate{     x: self.bottom_right_front.x,  y: self.bottom_right_front.y,    z: other.top_left_back.z -1}},
        

                // Middle layer 
                Cuboid{ top_left_back: Coordinate{      x: self.top_left_back.x,       y: self.top_left_back.y,         z: other.top_left_back.z},
                        bottom_right_front: Coordinate{ x: self.bottom_right_front.x,  y: other.top_left_back.y -1,     z: other.bottom_right_front.z}},

                        
                Cuboid{ top_left_back: Coordinate{      x: self.top_left_back.x,        y: other.top_left_back.y,       z: other.top_left_back.z },
                        bottom_right_front: Coordinate{ x: other.top_left_back.x -1,    y: other.bottom_right_front.y,  z: other.bottom_right_front.z}},


                Cuboid{ top_left_back: Coordinate{      x: other.bottom_right_front.x+1,  y: other.top_left_back.y,     z: other.top_left_back.z },
                        bottom_right_front: Coordinate{ x: self.bottom_right_front.x , y: other.bottom_right_front.y ,  z: other.bottom_right_front.z}},
                
                Cuboid{ top_left_back: Coordinate{      x: self.top_left_back.x,       y: other.bottom_right_front.y+1, z: other.top_left_back.z },
                        bottom_right_front: Coordinate{ x: self.bottom_right_front.x , y: self.bottom_right_front.y ,   z: other.bottom_right_front.z }},


                // top layer
                Cuboid{ top_left_back: Coordinate{  x: self.top_left_back.x,       y: self.top_left_back.y,       z: other.bottom_right_front.z+1},
                    bottom_right_front: Coordinate{ x: self.bottom_right_front.x , y: self.bottom_right_front.y,  z: self.bottom_right_front.z}},
        

            ];

            tentative.iter()
                .filter(|c| c.top_left_back.x <= c.bottom_right_front.x && c.top_left_back.y <= c.bottom_right_front.y &&  c.top_left_back.z <= c.bottom_right_front.z).copied()
                .collect::<Vec<_>>()
        } else {
            vec![*self]
        }
    }



    fn volume(&self) -> isize {
        (self.bottom_right_front.x - self.top_left_back.x +1) *  (self.bottom_right_front.y - self.top_left_back.y +1) * (self.bottom_right_front.z - self.top_left_back.z +1)
    }
}
fn main() {
    let data =  read_to_string("./input.txt").unwrap();
    reboot_reactor(&data, false);
    reboot_reactor(&data, true);
}

fn reboot_reactor(data: &str, part2: bool) {

    let mut reactor: Vec<Cuboid> = Vec::new();

    for line in data.split('\n') {
        let mut tokens = line.split_ascii_whitespace();
        let state = tokens.next().unwrap();
        let bounding_box = tokens.next().unwrap();
        let mut coords = bounding_box.split(',');
        let (start_x, end_x) = coords_to_min_max(coords.next().unwrap());
        let (start_y, end_y) = coords_to_min_max(coords.next().unwrap());
        let (start_z, end_z) = coords_to_min_max(coords.next().unwrap());
        let top_left_back = Coordinate{x:start_x, y: start_y, z: start_z};
        let bottom_right_front = Coordinate{x: end_x, y: end_y, z: end_z};
        if start_x.abs() < 50 || part2 {
            match state {
                "on" => {
                    reactor = remove_points(reactor, Cuboid{top_left_back, bottom_right_front});
                    reactor.push(Cuboid{top_left_back, bottom_right_front})
                        },
                "off" => { 
                    reactor = remove_points(reactor, Cuboid{top_left_back, bottom_right_front});},
                _ => unreachable!()
            };
        }
    }
    
    println!("Lit cubes: {:?}", reactor.iter().map(|x| x.volume()).sum::<isize>());
}


fn coords_to_min_max( axis: &str) -> (isize,isize) {

    let mut nums = axis[2..].split("..");
    (nums.next().unwrap().parse::<isize>().unwrap(),nums.next().unwrap().parse::<isize>().unwrap())
}

fn remove_points(reactor: Vec<Cuboid>, block_to_remove:Cuboid) -> Vec<Cuboid> {
    let mut new_reactor: Vec<Cuboid> = Vec::new();
    for cuboid in reactor {
        if let Some(intersection) = cuboid.intersection(&block_to_remove) {
            let mut new_cubes = cuboid.difference(&intersection);
            new_reactor.append(&mut new_cubes);

        } else {
            new_reactor.push(cuboid);

        }
    }
    new_reactor
}