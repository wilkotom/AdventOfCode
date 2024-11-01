use std::{collections::{HashSet, BTreeSet}, fs::read_to_string};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Coordinate {
    x: i32,
    y: i32, 
    z: i32
}

impl Coordinate {
    fn neighbours(&self, part2: bool ) -> Vec<Self>{
        if !part2 {
            [Coordinate{x: self.x -1, y: self.y, z: self.z},
            Coordinate{x: self.x +1, y: self.y, z: self.z},
            Coordinate{x: self.x, y: self.y -1, z: self.z },
            Coordinate{x: self.x, y: self.y +1, z: self.z}]
            .iter()
            .filter(|c| c.x >= 0 && c.x <= 4 && c.y >=0  && c.y <=4)
            .copied()
            .collect::<Vec<_>>()
        } else {
            match (self.x, self.y) {
                (0,0) => { vec![Coordinate{x: 1, y: 0, z: self.z}, 
                                Coordinate{x: 0, y:1, z: self.z},
                                Coordinate{x: 2, y:1, z: self.z -1},
                                Coordinate{x: 1, y:2, z: self.z -1}] },

                (4,0) => { vec![Coordinate{x: 3, y: 0, z: self.z}, 
                                Coordinate{x: 4, y:1, z: self.z},
                                Coordinate{x: 2, y:1, z: self.z -1},
                                Coordinate{x: 3, y:2, z: self.z -1}] },

                (0,4) => { vec![Coordinate{x: 1, y: 4, z: self.z}, 
                                Coordinate{x: 0, y: 3, z: self.z},
                                Coordinate{x: 1, y: 2, z: self.z -1},
                                Coordinate{x: 2, y: 3, z: self.z -1}] },

                (4,4) => { vec![Coordinate{x:3, y:4, z: self.z},
                                Coordinate{x:4, y:3, z: self.z},
                                Coordinate{x:3, y:2, z: self.z -1},
                                Coordinate{x:2, y:3, z: self.z -1} ]},

                (x,0) => {
                            vec![ Coordinate{x: x - 1, y: 0, z: self.z},
                                Coordinate{x: x + 1, y: 0, z: self.z},
                                Coordinate{x , y: 1, z: self.z},
                                Coordinate{x: 2, y: 1, z: self.z -1}]},
                
                (x,4) => {
                            vec![ Coordinate{x: x - 1, y: 4, z: self.z},
                                Coordinate{x: x + 1, y: 4, z: self.z},
                                Coordinate{x , y: 3, z: self.z},
                                Coordinate{x: 2, y: 3, z: self.z -1}]},

                (0,y) => {
                            vec![ Coordinate{x: 0, y: y -1, z: self.z},
                                Coordinate{x: 0, y: y+1 , z: self.z},
                                Coordinate{x: self.x +1 , y, z: self.z},
                                Coordinate{x: 1, y: 2, z: self.z -1}]},

                (4,y) => {
                            vec![ Coordinate{x: 4, y: y -1, z: self.z},
                                Coordinate{x: 4, y: y+1 , z: self.z},
                                Coordinate{x: self.x -1 , y, z: self.z},
                                Coordinate{x: 3, y: 2, z: self.z -1}]},

                (2,1) => {
                    vec![Coordinate{x:1, y:1, z: self.z},
                        Coordinate{x:3, y:1, z: self.z},
                        Coordinate{x:2, y:0, z: self.z},
                        Coordinate{x:0, y: 0, z: self.z +1},
                        Coordinate{x:1, y: 0, z: self.z +1},
                        Coordinate{x:2, y: 0, z: self.z +1},
                        Coordinate{x:3, y: 0, z: self.z +1},
                        Coordinate{x:4, y: 0, z: self.z +1}]
                },

                (1,2) => {
                    vec![Coordinate{x:1, y:1, z: self.z},
                        Coordinate{x:1, y:3, z: self.z},
                        Coordinate{x:0, y:2, z: self.z},
                        Coordinate{x:0, y: 0, z: self.z +1},
                        Coordinate{x:0, y: 1, z: self.z +1},
                        Coordinate{x:0, y: 2, z: self.z +1},
                        Coordinate{x:0, y: 3, z: self.z +1},
                        Coordinate{x:0, y: 4, z: self.z +1}]
                },
                (3,2) => {
                    vec![Coordinate{x:3, y:1, z: self.z},
                        Coordinate{x:3, y:3, z: self.z},
                        Coordinate{x:4, y:2, z: self.z},
                        Coordinate{x:4, y: 0, z: self.z +1},
                        Coordinate{x:4, y: 1, z: self.z +1},
                        Coordinate{x:4, y: 2, z: self.z +1},
                        Coordinate{x:4, y: 3, z: self.z +1},
                        Coordinate{x:4, y: 4, z: self.z +1}]
                },
                (2,3) => {
                    vec![Coordinate{x:1, y:3, z: self.z},
                        Coordinate{x:3, y:3, z: self.z},
                        Coordinate{x:2, y:4, z: self.z},
                        Coordinate{x:0, y: 4, z: self.z +1},
                        Coordinate{x:1, y: 4, z: self.z +1},
                        Coordinate{x:2, y: 4, z: self.z +1},
                        Coordinate{x:3, y: 4, z: self.z +1},
                        Coordinate{x:4, y: 4, z: self.z +1}]
                }

                _ => {
                    vec![Coordinate{x: self.x -1, y: self.y, z: self.z},
                    Coordinate{x: self.x +1, y: self.y, z: self.z},
                    Coordinate{x: self.x, y: self.y -1, z: self.z },
                    Coordinate{x: self.x, y: self.y +1, z: self.z}]
                }

                
            }
        }
    }
}

fn main() {
    let mut grid = read_grid(&read_to_string("./day24/input.txt").unwrap());
    println!("Part 1: {}", part1(grid.clone()));
    for _ in 0..200 {
        grid = generation_part1(&grid, true);
    }
    println!("Part 2: {}", grid.len());

}


fn part1(mut grid: HashSet<Coordinate>) -> i32 {
    let mut seen = BTreeSet::new();
    let mut biodiversity = get_biodiversity(&grid);
    while ! seen.contains(&biodiversity) {

        seen.insert(biodiversity);
        grid = generation_part1(&grid, false);
        biodiversity = get_biodiversity(&grid);
    }
    biodiversity
}

fn generation_part1(grid: &HashSet<Coordinate>, part2: bool) -> HashSet<Coordinate>{
    let mut potential = grid.clone();
    for cell in grid {
        for neighbour in cell.neighbours(part2) {
            potential.insert(neighbour);
        }
    }
    let mut new_grid = HashSet::new();

    for cell in potential {
        let neighbour_count = cell.neighbours(part2).iter().filter(|c| grid.contains(c)).count();
        if neighbour_count == 1 || ((! grid.contains(&cell)) && neighbour_count == 2) {
            new_grid.insert(cell);
            
        }
    }

    new_grid
}

fn read_grid(data: &str) -> HashSet<Coordinate> {
    // let data = read_to_string("./day24/input.txt").unwrap();
    let mut bugs = HashSet::new();
    for (y, line) in data.split('\n').enumerate() {
        for (x,c) in line.chars().enumerate() {
            if c == '#' {
                bugs.insert(Coordinate{x: x as i32, y: y as i32, z: 0});
            }
        }
    }
    bugs
}

fn get_biodiversity(grid: &HashSet<Coordinate>) -> i32{
    let mut total = 0;
    for y in (0..=4).rev() {
        for x in (0..=4).rev() {
            total <<= 1;
            total += if grid.contains(&Coordinate{x, y, z:0}) {1} else {0};
        }
    }
    total
}



#[test]
fn validate_biodiversity() {
    assert_eq!(get_biodiversity(&read_grid(".....\n.....\n.....\n#....\n.#...")), 2129920);
}
