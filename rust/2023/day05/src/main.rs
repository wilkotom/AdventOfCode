use std::error::Error;
use aochelpers::get_daily_input;


#[derive(Debug,PartialEq, Copy, Clone)]
struct Range {
    start: i64,
    end: i64
}

#[derive(Debug,PartialEq, Copy, Clone)]
struct AlmanacLine {
    delta: i64,
    source_range: Range,
}

struct Almanac {
    seed_list: Vec<i64>,
    seed_ranges: Vec<Range>,
    mappings: Vec<Vec<AlmanacLine>>
}


fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(5,2023)?;
    let almanac = parse_almanac(&data);
    println!("Part 1: {}", part1(&almanac));
    println!("Part 2: {}", part2(&almanac));

    Ok(())
}
fn part1(almanac: &Almanac) -> i64 {
    almanac.seed_list.iter().map(|s| grow_seed(*s, almanac)).min().unwrap_or(0)
}

fn part2(almanac: &Almanac) -> i64 {
    // For each range of seeds, apply the next transformation
    let mut working_ranges = almanac.seed_ranges.clone();
    for mapping_set in almanac.mappings[..].iter() {
        let mut next_ranges: Vec<_> = Vec::new();
        while let Some(range) = working_ranges.pop() {
            let mut unmatched = true;
            for mapping in mapping_set {
                let (mut to_reevaluate, mut next_stage_ranges) = split_step_ranges(range, mapping);
                working_ranges.append(&mut to_reevaluate);
                if !next_stage_ranges.is_empty() {
                    unmatched = false;
                    next_ranges.append(&mut next_stage_ranges);
                }
            }
            if unmatched {
                next_ranges.push(range)
            }
        }
        working_ranges = next_ranges;
    }
    working_ranges.iter().map(|x| x.start).min().unwrap_or(0)
}

fn parse_almanac_entry(entry: &str) -> Vec<AlmanacLine> {
    let mut lines: std::str::Split<'_, char> = entry.split('\n');
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
    let seed_ranges = seed_list.chunks(2).map(|v| Range{start: v[0], end: v[0]+ v[1] -1}).collect();
    let mappings = entries.map(parse_almanac_entry).collect::<Vec<_>>();
    Almanac{seed_list, seed_ranges, mappings}
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

fn split_step_ranges(item_range: Range, transformation: &AlmanacLine) -> (Vec<Range>,Vec<Range>) {
    let mut new_ranges = Vec::new();
    let mut to_reevaluate = Vec::new();
    if item_range.end < transformation.source_range.start || transformation.source_range.end < item_range.start {
        // do nothing
    } else if item_range.start >= transformation.source_range.start && item_range.end <=transformation.source_range.end {
        /*     
               <- item ->
            <---- tran ---->
        */
        new_ranges.push(Range{start: transformation.delta + item_range.start, end: transformation.delta + item_range.end});
    } else if item_range.start < transformation.source_range.start && item_range.end > transformation.source_range.end {
        /*     
            <---- item ---->
                <- tran ->
        */
        to_reevaluate.push(Range{start: item_range.start, end: transformation.source_range.start -1});
        new_ranges.push(Range{start: transformation.source_range.start + transformation.delta, end: transformation.source_range.end + transformation.delta});
        to_reevaluate.push(Range{start: transformation.source_range.end +1, end: item_range.end});
    } else if item_range.start < transformation.source_range.start && item_range.end <= transformation.source_range.end {
        /*
            <---- item ---->
                    <---- tran ---->
            */
        to_reevaluate.push(Range{start: item_range.start, end: transformation.source_range.start -1});
        new_ranges.push(Range{start: transformation.delta + transformation.source_range.start, end: transformation.delta + item_range.end});
    } else if item_range.start >= transformation.source_range.start && item_range.end > transformation.source_range.end {
        /*     
                <---- item ---->
            <---- tran ---->
        */
        new_ranges.push(Range{start: transformation.delta + item_range.start, end: transformation.delta + transformation.source_range.end});
        to_reevaluate.push(Range{start: transformation.source_range.end +1, end: item_range.end});
    } 

    (to_reevaluate, new_ranges)
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
    fn test_parse_almanac_entry() {
        assert_eq!(parse_almanac_entry("seed-to-soil map:
50 98 2
52 50 48"), vec![AlmanacLine{source_range: Range{start: 98, end: 99},delta: -48 }, 
                AlmanacLine{source_range: Range{start: 50, end: 97}, delta: 2 }])
    }

    #[test]
    fn test_parse_seeds() {
        let almanac = parse_almanac(DATA);
        assert_eq!(almanac.seed_list, vec![79,14,55,13]);
        assert_eq!(almanac.seed_ranges, vec![Range{start: 79, end: 92}, Range{start: 55, end: 67}]);
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