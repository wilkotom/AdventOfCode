use std::collections::HashSet;

fn main() {
    // println!("{:?}", gen_neighbours(vec![0,0,0]));
    let cells = read_starting_state("./input.txt".to_string(),3);

    let cells = generation(cells.clone(),6);
    println!("3 dimensional answer: {:?}", cells.len());

    let cells = read_starting_state("./input.txt".to_string(),4);

    let cells = generation(cells.clone(),6);
    println!("4 dimensional answer: {:?}", cells.len());


}

fn read_starting_state(filename: String, dimensions: usize) -> HashSet<Vec<i32>> {
    let mut cells = HashSet::new();
    let file = std::fs::read_to_string(filename).unwrap_or("".to_string());
    let data: Vec<_> = file.split("\n").collect();
    for y in 0..data.len() {
        let line = data.get(y).unwrap();
        for x in 0..line.len() {
            if line.chars().nth(x).unwrap() == '#' {
                let mut cell = Vec::new();
                cell.push(x as i32);
                cell.push(y as i32);
                while cell.len() < dimensions {
                    cell.push(0);
                }
                cells.insert(cell);
            }
        }
    }

    cells
}


fn generation(cubes: HashSet<Vec<i32>>, generations: i32) -> HashSet<Vec<i32>> {
    if generations > 0 {
        let mut next_cubes = HashSet::new();
        let mut possible_next_cubes: HashSet<Vec<i32>> = HashSet::new();
        for cube in cubes.iter() {
            possible_next_cubes.insert(cube.clone());
            for n in gen_neighbours(&cube).clone() {
                possible_next_cubes.insert(n);
            }
        }
        for possible_cube in possible_next_cubes {
            let mut neighbour_count = 0;
            for neighbour in gen_neighbours(&possible_cube.clone()) {
                if cubes.contains(&neighbour) {
                    neighbour_count += 1;
                }
            }
            if neighbour_count == 3 || neighbour_count == 2 && cubes.contains(&possible_cube) {
                next_cubes.insert(possible_cube);
            }
        }
        generation(next_cubes, generations -1)
    }
    else {
        cubes
    }
}



fn gen_neighbours( coordinate: &Vec<i32>) -> HashSet<Vec<i32>> {
    let offsets = gen_neighbours_coords(coordinate.len() as i32);
    let mut neighbours = HashSet::new();
    for offset in offsets.iter() {
        if offset.iter().all(|&x| x == 0) {
            continue;
        }
        let mut new_coord = Vec::new();
        for i in 0..coordinate.len() {
            new_coord.push(coordinate[i] + offset[i]);
        }
        neighbours.insert(new_coord);
    }

    neighbours
}


fn gen_neighbours_coords(dimension: i32) -> Vec<Vec<i32>> {
    match dimension {
        d if d <= 0 => Vec::new(),
        1 => vec![vec![-1], vec![0], vec![1]], 
        _ => {
            let mut result = Vec::new();
            for lower_dimensional_neighbour in gen_neighbours_coords(dimension -1) {
                for n in -1..2 {
                    let mut dim = lower_dimensional_neighbour.clone();
                    dim.push(n);
                    result.push(dim);
                }
            };
            result
        }
    }
}