
fn main() {
    let serial_number = 9110;
    let grid = generate_grid(serial_number);
    find_best_locations(&grid);
}


fn find_best_locations(grid: &[[i64; 301];301])  {
   let mut last_grid = *grid;
   let mut best_score_part1 = i64::MIN;
   let mut winning_location_part1 = (0,0);
   let mut best_score = i64::MIN;
   let mut winning_location = (0,0,0);

    for size in 1..=300 {
        let mut new_grid = [[0_i64;301];301];
        for x in 1..(300-size) {
            for y in 1..(300-size){
                let right_edge = (y..=y+size-1)
                    .map(|y1| 
                        grid[x+size-1][y1]).sum::<i64>();
                let bottom_edge = (x..x+size-1).map(|x1| grid[x1][y+size-1]).sum::<i64>();
                let square = last_grid[x][y];
                let score = square + bottom_edge + right_edge;
                new_grid[x][y] =  score;
                if size == 3 && score > best_score_part1 {
                    best_score_part1 = score;
                    winning_location_part1 = (x,y);
                    
                }
                if score > best_score {
                    best_score = score;
                    winning_location = (x,y,size);
                }
            }
        }
        last_grid = new_grid;
    }

    println!("Part 1: {:?}", winning_location_part1);
    println!("Part 2: {:?}", winning_location);
    }

fn generate_grid(serial: i64) -> [[i64; 301];301] {
    // Pad the top and left edges with a 0 to avoid all the annoying 1-indexing bugs of a true 300x300 grid.
    let mut grid = [[0;301];301];  

    for y in 1..=300 {
        for x in 1..=300 {
            grid[x as usize][y as usize] = power_level(x,y,serial);
        }
    }

    grid
}

fn power_level(x: i64, y: i64, serial: i64) -> i64 {
    ((((x+10) * y) + serial)* (x+10)) /100 % 10 -5
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3_5_8() {
        assert_eq!(power_level(3,5,8), 4);
    }

    #[test]
    fn test_122_79_57() {
        assert_eq!(power_level(122,79,57), -5);
    }

    #[test]
    fn test_217_196_39() {
        assert_eq!(power_level(217,196,39), 0);
    }

    #[test]
    fn test_101_153_71() {
        assert_eq!(power_level(101,153,71), 4);
    }
}
