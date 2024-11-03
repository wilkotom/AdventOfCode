use std::error::Error;
use aochelpers::{get_daily_input, Coordinate3d};

#[derive(Debug, Clone, Copy)]
struct Hailstone {
    position: Coordinate3d<f64>,
    velocity: Coordinate3d<f64>
}

fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(24,2023)?;
    let hailstones: Vec<Hailstone> = parse_data(&data);
    println!("Part 1: {}", part1(&hailstones, 200000000000000.0, 400000000000000.0));
    println!("Part 2: {}", part2(&hailstones));

    Ok(())
}


fn collision_point_2d(a: &Hailstone, b: &Hailstone, min_bound: f64) -> Option<Coordinate3d<f64>>{
    let dya = a.velocity.y / a.velocity.x;
    let ay_for_zero_x = a.position.y - dya * a.position.x;
    let dyb = b.velocity.y / b.velocity.x;
    let by_for_zero_x = b.position.y - dyb * b.position.x;

    if dya == dyb {
        if ay_for_zero_x == by_for_zero_x {
            return Some(Coordinate3d{x: min_bound+1.0, y: min_bound+1.0, z:0.0});
        }
        return None;
    }
    let intersection_x = (by_for_zero_x-ay_for_zero_x)/(dya-dyb);
    let intersection_y = intersection_x* dya+ay_for_zero_x;

    if ( (intersection_x > a.position.x ) == (a.velocity.x >0.0)) && ((intersection_x > b.position.x ) == (b.velocity.x >0.0)) {
        return Some(Coordinate3d{x: intersection_x, y: intersection_y, z: 0.0})
    }
None


}


fn solve_part2(a: Hailstone, b: Hailstone, candidate_velocity: Coordinate3d<f64>) -> Option<(f64, f64, Coordinate3d<f64>)> {
    let t2_numerator = b.position.y - a.position.y - (((a.velocity.y - candidate_velocity.y) * (b.position.x - a.position.x)) / (a.velocity.x - candidate_velocity.x));
    let t2_denominator = candidate_velocity.y - b.velocity.y - (((a.velocity.y - candidate_velocity.y) * (candidate_velocity.x - b.velocity.x)) / (a.velocity.x - candidate_velocity.x));

    let t2 = t2_numerator / t2_denominator;

    let t1 = (b.position.x - a.position.x - t2 * (candidate_velocity.x - b.velocity.x)) / (a.velocity.x - candidate_velocity.x);

    let px = a.position.x - t1 * (candidate_velocity.x - a.velocity.x);
    let py = a.position.y - t1 * (candidate_velocity.y - a.velocity.y);
    let pz = a.position.z - t1 * (candidate_velocity.z - a.velocity.z);

    if (pz + t2 * (candidate_velocity.z - b.velocity.z) - b.position.z).abs() > 0.0001 {
        None
    } else {
        Some((
            t1,
            t2,
            Coordinate3d {
                x: px,
                y: py,
                z: pz,
            },
        ))
    }
}


fn part2(hailstones: &[Hailstone]) -> usize {

    let a = hailstones[0];
    let b = hailstones[1];

    let is_int = |f: f64| (f.round() - f).abs() < 0.0001;

    for vx in -500..500 {
        for vy in -500..500 {
            'outer: for vz in -500..500 {
                let vx = vx as f64;
                let vy = vy as f64;
                let vz = vz as f64;

                if let Some((
                    t1,
                    t2,
                    Coordinate3d {
                        x: px,
                        y: py,
                        z: pz,
                    },
                )) = solve_part2(a, b, Coordinate3d{x: vx, y: vy, z: vz})
                {
                    if !(t1.is_finite()
                        && t2.is_finite()
                        && px.is_finite()
                        && py.is_finite()
                        && pz.is_finite())
                    {
                        continue;
                    }

                    if t1.is_sign_negative() || t2.is_sign_negative() {
                        continue;
                    }

                    if !(is_int(t1) && is_int(t2) && is_int(px) && is_int(py) && is_int(pz)) {
                        continue;
                    }

                    for c in hailstones.iter().skip(2) {

                        let t3 = (c.position.x - px) / (vx - c.velocity.x);

                        if (py + t3 * vy - (c.position.y + t3 * c.velocity.y)).abs() > 0.0001
                            || (pz + t3 * vz - (c.position.z + t3 * c.velocity.z)).abs() > 0.0001
                        {
                            continue 'outer;
                        }
                    }

                    return px as usize + py as usize + pz as usize;
                }
            }
        }
    }

    panic!("found no solution");
}
 
fn part1(hailstones: &[Hailstone], min_bound: f64, max_bound: f64) -> i128 {
    let mut result = 0;
    for( i, hailstone) in hailstones.iter().enumerate() {
        for other in hailstones[i+1..]. iter() {
            if let Some(collision_point) = collision_point_2d(hailstone, other, max_bound) {
                if collision_point.x >= min_bound && collision_point.x <= max_bound &&
                        collision_point.y >= min_bound && collision_point.y <= max_bound {
                    result+=1
                }
            }
        }
    }
    result
}

fn parse_data(data: &str) -> Vec<Hailstone> {
    let mut hailstones = Vec::new();
    for line in data.lines() {
        let mut sections = line.split(" @ ");
        let pos_section = sections.next().unwrap();
        let vel_section = sections.next().unwrap();
        let mut position = pos_section.split(", ").map(|x| x.parse::<f64>().unwrap());
        let mut velocity = vel_section.split(", ").map(|x| x.parse::<f64>().unwrap());
        let position = Coordinate3d{
            x: position.next().unwrap(), y: position.next().unwrap(), z: position.next().unwrap()
        };
        let velocity = Coordinate3d{
            x: velocity.next().unwrap(),
            y: velocity.next().unwrap(),
            z: velocity.next().unwrap()

        };
        hailstones.push(Hailstone{ position, velocity});


    }

    hailstones
}

#[cfg(test)]
mod tests {

    use super::*;

    const DATA: &str = 
"19, 13, 30 @ -2, 1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @ 1, -5, -3";

    #[test]
    fn test_parser() {
        println!("{:?}", parse_data(DATA));
    }

    #[test]
    fn test_part1(){
        let hailstones: Vec<Hailstone> = parse_data(DATA);
        assert_eq!(part1(&hailstones, 7.0, 27.0), 2); 
    }
}