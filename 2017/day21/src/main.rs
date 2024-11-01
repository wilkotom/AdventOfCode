use std::collections::HashMap;
#[derive(PartialEq, Eq,Hash,Debug,Clone)]
struct Grid {
    rows: Vec<Vec<bool>>
}

impl Grid {
    fn flip(&mut self) {
        self.rows.reverse();
    }

    fn rotate_clockwise(&mut self) {
        let mut new_grid: Vec<Vec<bool>> =Vec::new();
        for col in 0..self.rows[0].len() {
            let mut new_line: Vec<bool> = Vec::new();
            for row in  0..self.rows.len() {
                new_line.insert(0,self.rows[row][col])
            }
            new_grid.push(new_line);
        }
        self.rows = new_grid
    }

}

fn main() {
    let data = std::fs::read_to_string("./input.txt").unwrap();
    let mut rules: HashMap<Grid, Grid> = HashMap::new();

    for line in data.split('\n') {
        let mut components = line.split(" => ");
        let start = components.next().unwrap().split('/').map(|l| l.chars().map(|c| c == '#').collect::<Vec<_>>()).collect::<Vec<_>>();
        let finish = components.next().unwrap().split('/').map(|l| l.chars().map(|c| c == '#').collect::<Vec<_>>()).collect::<Vec<_>>();
        rules.insert(Grid{rows:start}, Grid{rows:finish});
    }

    let mut grid = vec![vec![false,true,false], vec![false, false, true], vec![true,true,true]];
    for _ in 0..18 {
        let mut next_grid = Vec::new();
        let step_size = if grid.len() % 2 == 0 { 2 } else { 3 };
        for row in (0.. grid.len()).step_by(step_size) {
            let mut next_rows: Vec<Vec<bool>> = vec![Vec::new(); step_size +1];
            'next_piece: for col in (0.. grid.len()).step_by(step_size) {

                let mut candidate_grid = Grid{rows: grid[row..row+step_size].iter().map(|r| r[col..col+step_size].to_vec()).collect::<Vec<_>>()};
                loop {
                    for _ in 0..4 {
                        if rules.contains_key(&candidate_grid) {
                            for (k, line) in rules.get(&candidate_grid).unwrap().rows.iter().clone().enumerate() {
                                let mut line = line.to_owned();
                                next_rows[k].append(&mut line);
                            }
                            continue 'next_piece;
                        }
                        candidate_grid.rotate_clockwise();
                    }
                    candidate_grid.flip();    
                }
                
            }
            next_grid.append(&mut next_rows);
        }
        grid = next_grid;
    }

    println!("Part 1: {:?}", grid.iter().map(|l| l.iter().filter(|k| **k).count()).sum::<usize>());
}
