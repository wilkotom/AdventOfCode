use std::error::Error;
use aochelpers::get_daily_input;


#[derive(Debug,PartialEq)]
struct Range {
    start: i64,
    end: i64
}

#[derive(Debug,PartialEq)]
struct AlmanacLine {
    delta: i64,
    source_range: Range,
}

struct Almanac {
    seed_list: Vec<i64>,
    mappings: Vec<Vec<AlmanacLine>>
}


fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(05,2023)?;
    let almanac = parse_almanac(&data);
    println!("Part 1: {}", part1(&almanac));
    println!("Part 2: {}", part2(&almanac));

    Ok(())
}
fn part1(almanac: &Almanac) -> i64 {
    almanac.seed_list.iter().map(|s| grow_seed(*s, almanac)).min().unwrap_or(0)
}

fn part2(almanac: &Almanac) -> i64 {
    // TODO: Refactor brute force
    let mut values = almanac.seed_list.iter();
    let mut result = i64::MAX;

    while let Some(start) = values.next() {
        let count = values.next().unwrap();
        result = result.min(grow_seeds(*start, *count, almanac));
    }
    result
}

fn grow_seeds(start:i64, range: i64, almanac: &Almanac) -> i64 {
    let mut result = i64::MAX;
    for i in start..start+range {
        result = result.min(grow_seed(i, almanac))
    }
    result
}

fn parse_almanac_entry(entry: &str) -> Vec<AlmanacLine> {
    let mut lines = entry.split('\n');
    let mut almanac_entry = Vec::new();
    lines.next();
    for line in lines {
        let mut numbers = line.split(" ").filter_map(|x| x.parse::<i64>().ok());
        let dest = numbers.next().unwrap();
        let source = numbers.next().unwrap();
        let offset = numbers.next().unwrap();
        almanac_entry.push(AlmanacLine{delta: dest - source ,source_range: Range { start: source, end: source + offset -1 }});
    }
    almanac_entry
}

fn parse_almanac(almanac: &str) -> Almanac {
    let mut entries = almanac.split("\n\n");
    let seed_list = entries.next().unwrap().split(' ').filter_map(|x| x.parse::<i64>().ok()).collect::<Vec<_>>();
    let mappings = entries.map(parse_almanac_entry).collect::<Vec<_>>();
    Almanac{seed_list, mappings}
}

fn grow_seed(mut id: i64, almanac: &Almanac) -> i64{
    for mapping in almanac.mappings.iter() {
        for entry in mapping {
            if id >= entry.source_range.start  && id <= entry.source_range.end{
                id += entry.delta;
                break;
            }
        }
    }
    id
}
/* 
fn combine_ranges(starting_range: Range, next_steps: Vec<Range>) -> Vec<Range> {
    let mut combined_ranges = vec![];
    for next_range in next_steps {
        if next_range.start > starting_range.end 
    }


    combined_ranges
}
*/
fn split_ranges(old: AlmanacLine, transformation: AlmanacLine) -> Vec<AlmanacLine> {

    if old.source_range.end < transformation.source_range.start || old.source_range.end > transformation.source_range.end {
        //ranges don't overlap  
        vec![old]
    
    } else if old.source_range.end >= transformation.source_range.start && old.source_range.end <= transformation.source_range.end {
        // old is entirely within transformation area
        vec![AlmanacLine{source_range: old.source_range, delta: old.delta + transformation.delta}]
    } else if old.source_range.start < transformation.source_range.start && old.source_range.end < transformation.source_range.end{
        // old overlaps transformation area to the left
        vec![
            AlmanacLine{source_range: Range{start: old.source_range.start, end: transformation.source_range.start -1 }, delta: old.delta},
            AlmanacLine{source_range: Range{start: transformation.source_range.start, end: old.source_range.end}, delta: old.delta + transformation.delta}
            ]
    } else if old.source_range.start <= 3 {
        // overlap to the right
        vec!()
    } else {
        vec![]
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

#[test]
    fn test_parser() {
        assert_eq!(parse_almanac_entry("seed-to-soil map:
50 98 2
52 50 48"), vec![AlmanacLine{source_range: Range{start: 98, end: 99},delta: -48 }, 
                AlmanacLine{source_range: Range{start: 50, end: 97}, delta: 2 }])
    }

    #[test]
    fn test_seed_79() {
        let almanac = parse_almanac(DATA);
        assert_eq!(grow_seed(79, &almanac), 82);
        assert_eq!(grow_seed(14, &almanac), 43);
        assert_eq!(grow_seed(55, &almanac), 86);
        assert_eq!(grow_seed(13, &almanac), 35);

    }

    #[test]
    fn test_part1() {
        let almanac = parse_almanac(DATA);
        assert_eq!(part1(&almanac), 35);
    }


    #[test]
    fn test_part2() {
        let almanac = parse_almanac(DATA);
        assert_eq!(part2(&almanac), 46);
    }
}