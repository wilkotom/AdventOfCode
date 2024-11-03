#[derive(Debug,Copy,Clone, PartialEq, Eq)]
enum Square {
    Empty,
    SouthFacing,
    EastFacing
}

fn main() {
    let data = std::fs::read_to_string("./input.txt").unwrap();
    let mut map: Vec<Vec<Square>> = Vec::new();

    for line in data.split('\n') {
        let mut gridline: Vec<Square> = Vec::new();
        for c in line.chars() {
            gridline.push( match c {
                'v' => Square::SouthFacing,
                '>' => Square::EastFacing,
                '.' => Square::Empty,
                _ => unreachable!()
            });
        }
        map.push(gridline);
    }
    print_grid(&map);

    let mut changed = true;
    let mut counter = 0;
    while changed {
        let t = generation(map);
        map = t.0;
        changed = t.1;
        counter +=1;
    }
    println!("{}", counter);

}

fn generation(grid: Vec<Vec<Square>>) -> (Vec<Vec<Square>>, bool) {

    let mut line = 0;
    // move right
    let mut inter_grid: Vec<Vec<Square>> = Vec::new();
    let grid_len = grid.len(); 
    let line_len = grid[0].len();
    let mut changed = false;
    while line < grid_len {
        let mut col = 0;
        let mut new_line: Vec<Square> = Vec::new();
        while col < grid[line].len() {
            if grid[line][col] == Square::EastFacing && grid[line][(col+1) % line_len] == Square::Empty {
                changed = true;
                new_line.push(Square::Empty);
                if (col+1) % line_len == 0 {
                    new_line[0] = Square::EastFacing;
                } else {
                    new_line.push(Square::EastFacing);
                }
                col += 2;
            } else {
                new_line.push(grid[line][col]);
                col += 1;
            }
            
        }
        inter_grid.push(new_line);
        line += 1;
    }

    let mut inserted: Vec<(usize,usize)> = Vec::new();
    let mut final_grid: Vec<Vec<Square>> = Vec::new();
    line = 0;

    while line < grid_len {
        let mut col = 0;
        let mut new_line: Vec<Square> = Vec::new();
        while col < grid[line].len() {
            if inter_grid[line][col] == Square::SouthFacing && inter_grid[(line+1) % grid_len][col] == Square::Empty {
                changed = true;
                new_line.push(Square::Empty);
                inserted.push(((line+1) % grid_len, col));
            } else {
                new_line.push(inter_grid[line][col]);
            }
            col +=1;
        }
        final_grid.push(new_line);
        line += 1;
    }
    for (row,col) in inserted {
        final_grid[row][col] = Square::SouthFacing;
    }
    (final_grid, changed)
}

fn print_grid(grid: &Vec<Vec<Square>>) {
    for line in grid {
        for item in line {
            print!("{}", match item {
                Square::Empty => '.',
                Square::SouthFacing => 'v',
                Square::EastFacing => '>',
            });
        }
        println!();
    }
}