use std::collections::{BinaryHeap, HashSet};
use aochelpers::{get_daily_input, ScoredItem};
use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(8,2025)?;
    let points = parse_data(&data);
    let (p1, p2) = solve(&points, 1000);
    println!("Part 1:{}", p1);
    println!("Part 2:{}", p2);
    Ok(())
}

fn solve(points: &[Vec<i64>], pair_count: usize) -> (usize, i64) {
    let mut pairs = BinaryHeap::new();
    for (i, p1) in points[..].iter().enumerate() {
        for p2 in points[i+1..].iter() {
            let cost = (p1[0] - p2[0]) * (p1[0] - p2[0]) +
                            (p1[1] - p2[1]) * (p1[1] - p2[1]) +
                            (p1[2] - p2[2]) * (p1[2] - p2[2]);
            pairs.push(ScoredItem{cost, item: (p1,p2)});
        }
    }
    let mut clusters: Vec<HashSet<_>> = Vec::new();
    let mut p1answer = 0;
    let mut p1counter = 0;
    let mut last_pair = None;
    while let Some(pair) = pairs.pop() && last_pair.is_none() {
        if p1counter == pair_count {
            clusters.sort_by_key(|b| std::cmp::Reverse(b.len()));
            p1answer = clusters[0].len() * clusters[1].len() * clusters[2].len();
        }
        let mut first_match = None;
        let mut second_match = None;

        for (i, cluster) in clusters.iter().enumerate() {
            if cluster.contains(pair.item.0) || cluster.contains(pair.item.1) {
                if first_match.is_none() {
                    first_match = Some(i);
                } else {
                    second_match = Some(i);
                    break;
                }
            }
        }
        match (first_match, second_match) {
            (None, None) => {clusters.push(HashSet::from([pair.item.0, pair.item.1]))},
            (Some(i), Some(j)) => {
                let merged = clusters.swap_remove(j);
                clusters[i].extend(merged);
            },
            (Some(i), None) => {
                clusters[i].insert(pair.item.0);
                clusters[i].insert(pair.item.1);
            },
            _ => unimplemented!(),
        }
        if clusters.len() == 1 && clusters[0].len() == points.len() {
            last_pair = Some(pair.item);
        }
        p1counter += 1;
    }
    (p1answer, if let Some(pair) = last_pair {
        pair.0[0] * pair.1[0]
    } else {
        0
    })
}


fn parse_data(data: &str) -> Vec<Vec<i64>> {
    data.lines().map(|l| l.split(',').flat_map(|v| v.parse()).take(3).collect::<Vec<_>>()).collect()
}

#[cfg(test)]
mod tests {
    use crate::{solve, parse_data};

    const TESTDATA: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_p1() {
        let data = parse_data(TESTDATA);
        let (p1, _) = solve(&data, 10);
        assert_eq!(p1, 40);
    }

    #[test]
    fn test_p2() {
        let data = parse_data(TESTDATA);
        let (_, p2) = solve(&data, 10);
        assert_eq!(p2, 25272);
    }
    
}
