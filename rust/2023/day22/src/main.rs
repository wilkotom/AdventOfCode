use std::{error::Error, collections::{HashSet}};
use aochelpers::{get_daily_input, Coordinate3d};

#[derive(Debug,Copy,Clone, Hash, PartialEq, Eq)]
struct Brick {
    start: Coordinate3d<usize>,
    end: Coordinate3d<usize>
}

// 1235 too low
fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(22,2023)?;
    let mut tower = parse_data(&data);
        
    let mut next_tower = gravity(&tower);
    while tower != next_tower {
        tower = next_tower;
        next_tower = gravity(&tower);
    }
    let results = part1(tower);
    println!("Part 1: {:?}", results.0);
    println!("Part 1: {:?}", results.1);
    Ok(())
}

fn part1(mut tower: Vec<Brick>) -> (usize, usize) {
    let mut result = 0;
    let mut next_tower = gravity(&tower);
    while tower != next_tower {
        tower = next_tower;
        next_tower = gravity(&tower);
    }
    let mut fallen_bricks = 0;
    for i in 0..tower.len() {
        let mut tower_with_brick_removed = tower.clone();
        tower_with_brick_removed.remove(i);
        let mut collapsed = gravity(&tower_with_brick_removed);
        if collapsed == tower_with_brick_removed {
            result += 1;
        } else {
            let mut further_collapsed = gravity(&collapsed);
            while further_collapsed != collapsed {
                collapsed = further_collapsed.clone();
                further_collapsed = gravity(&further_collapsed);
            } 
            let bricks_fallen_this_time = tower_with_brick_removed.iter().enumerate().filter(|(i,c)| **c != further_collapsed[*i]).count();
            fallen_bricks += bricks_fallen_this_time;
        }
    }
    (result, fallen_bricks)
}

fn parse_data(data: &str) -> Vec<Brick> {
    let mut tower = Vec::new();
    for line in data.lines() {
        let mut coords = line.split('~');
        let mut first_coords = coords.next().unwrap().split(',');
        let mut second_coords = coords.next().unwrap().split(',');
        let mut brick = Brick {
            start: Coordinate3d{
                x: first_coords.next().unwrap().parse::<usize>().unwrap(),
                y: first_coords.next().unwrap().parse::<usize>().unwrap(),
                z: first_coords.next().unwrap().parse::<usize>().unwrap(),
            },
            end: Coordinate3d{
                x: second_coords.next().unwrap().parse::<usize>().unwrap(),
                y: second_coords.next().unwrap().parse::<usize>().unwrap(),
                z: second_coords.next().unwrap().parse::<usize>().unwrap(),
            }
        };
        if brick.end.x < brick.start.x || brick.end.y < brick.start.y || brick.end.z < brick.start.z {
            (brick.start, brick.end) = (brick.end, brick.start)
        }

        tower.push(brick);
    }
    tower
}

fn gravity(tower: &Vec<Brick>) -> Vec<Brick>{
    let mut sand_pile = HashSet::new();
    let mut moved = Vec::new();
    for brick in tower {
        for x in brick.start.x..=brick.end.x {
            for y in brick.start.y..=brick.end.y {
                for z in brick.start.z..=brick.end.z {
                    sand_pile.insert(Coordinate3d{x,y,z});
                }
            }
        }
    }

    for brick in tower {
        let mut can_move = 1;
        for x in brick.start.x..=brick.end.x {
            for y in brick.start.y ..= brick.end.y {
                if brick.start.z -1 == 0 || sand_pile.contains(&Coordinate3d{x,y, z: brick.start.z -1}) {
                    can_move = 0;
                }
            }
        }
        moved.push(Brick{
            start: Coordinate3d{x: brick.start.x, y: brick.start.y, z: brick.start.z - can_move},
            end: Coordinate3d{x: brick.end.x, y: brick.end.y, z: brick.end.z - can_move},
        });
    }
    moved
}

#[cfg(test)]
mod tests {
    use super::*; 
    const EXAMPLE1:&str = 
"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";


    #[test]
    fn test_example1() {
        let  tower = parse_data(EXAMPLE1);
        assert_eq!(part1(tower), (5, 7))


        
    }
}