use aochelpers::get_daily_input;
use std::{collections::HashMap, error::Error};


fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(11, 2025)?;
    let parsed = parse_data(&data);
    println!("Part 1: {}", path_count("you", "out", &parsed, &mut HashMap::new()));
    println!("Part 2: {}", part2(&parsed));

    Ok(())}

fn parse_data<'a>(data: &'a str) -> HashMap<&'a str, Vec<&'a str>> {
    let mut directions = HashMap::new();
    for line in data.lines() {
        let mut tokens = line.split_ascii_whitespace();
        let location = tokens.next().unwrap_or("unreachable:").strip_suffix(":").unwrap();
        directions.insert(location, tokens.collect());
    }
    directions
}

fn part2<'a>(paths: &HashMap<&'a str, Vec<&'a str>>) -> usize {
    let dac_fft = path_count("dac", "fft", paths, &mut HashMap::new());
    if dac_fft != 0 {
        path_count("svr", "dac", paths, &mut HashMap::new()) * 
        dac_fft *
        path_count("fft", "out", paths, &mut HashMap::new())
    } else {
        path_count("svr", "fft", paths, &mut HashMap::new()) * 
        path_count("fft", "dac", paths, &mut HashMap::new()) *
        path_count("dac", "out", paths, &mut HashMap::new())
    }
}

fn path_count<'a>(location: &'a str, destination: &'a str, paths: &HashMap<&'a str, Vec<&'a str>>, cache: &mut HashMap<&'a str, usize>) -> usize {
    if location == destination {
        1
    } else if let Some(&answer) = cache.get(location) {
        answer
    } else {
        let answer = paths.get(location).unwrap_or(&Vec::new()).iter().map(|d| path_count(d, destination, paths, cache)).sum();
        cache.insert(location, answer);
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TESTDATA: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    const P2TESTDATA: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn test_p1() {
        let parsed = parse_data(TESTDATA);
        assert_eq!(path_count("you", "out", &parsed, &mut HashMap::new()), 5);
    }

    #[test]
    fn test_svr_dac() {
        let parsed = parse_data(P2TESTDATA);
        assert_eq!(path_count("svr", "fft", &parsed, &mut HashMap::new()), 1);
    }

    #[test]
    fn test_dac_fft() {
        let parsed = parse_data(P2TESTDATA);
        assert_eq!(path_count("dac", "fft", &parsed, &mut HashMap::new()), 0);
    }
    #[test]
    fn test_fft_dac() {
        let parsed = parse_data(P2TESTDATA);
        assert_eq!(path_count("fft", "dac", &parsed, &mut HashMap::new()), 1);
    }

    #[test]
    fn test_p2() {
        let parsed = parse_data(P2TESTDATA);
        assert_eq!(part2(&parsed), 2);
    }
    
    
}