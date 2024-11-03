use std::{iter::once, error::Error, collections::HashMap};
use aochelpers::get_daily_input;

fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(12,2023)?;
    let plans = parse_data(&data);
    println!("Part 1: {}", part1(&plans));
    println!("Part 2: {}", part2(&plans));
    Ok(())
}

fn part1(plans: &[(String,Vec<usize>)]) -> usize {
    let mut cache: HashMap<(String,Vec<usize>), usize> = HashMap::new();
    plans.iter().map(|v: &(String, Vec<usize>)|validate_springs(&v.0,&v.1, &mut cache)).sum::<usize>()
}

fn part2(plans: &Vec<(String,Vec<usize>)>) -> usize {
    let mut total = 0;
    let mut cache: HashMap<(String,Vec<usize>), usize> = HashMap::new();
    for (springs, counts) in plans {
        let new_springs = springs.chars().chain(once('?')).cycle().take(5 * (springs.len() + 1) - 1).collect::<String>();
        let new_counts: Vec<_> = counts.iter().cycle().take(counts.len() * 5).copied().collect();
        let result = validate_springs(&new_springs,&new_counts, &mut cache);
        total += result

    }
    total

}
fn parse_data(data: &str) -> Vec<(String,Vec<usize>)> {

    data.split('\n').map(|l| {
        let mut f = l.split(' '); 
        let springs = f.next().unwrap().to_string();
        let groups  = f.next().unwrap().split(',').map(|v| v.parse::<usize>().unwrap()).collect::<Vec<_>>();
        (springs, groups)
    }).collect::<Vec<_>>()
}

fn validate_springs(springs: &str, counts: &[usize], cache: &mut HashMap<(String,Vec<usize>), usize>) -> usize {

    let cache_key = (springs.to_string(), counts.to_vec());
    let result = if let Some(res) = cache.get(&cache_key) {
        return *res
    } else if let Some(word) = springs.strip_prefix('.') {
        validate_springs(word, counts, cache)
    } else if let Some(stripped) = springs.strip_prefix('?') {
        let unknown_is_spring: String = springs.replacen('?', "#", 1);
        validate_springs(&unknown_is_spring, counts, cache) + validate_springs(stripped, counts, cache)
    } else if springs.is_empty() {
        counts.is_empty() as usize
    } else if counts.is_empty() && springs.contains('#') || 
            springs.len() < counts[0] ||
            springs[..counts[0]].contains('.') {
        0
    } else if springs.len() == counts[0] {
        (counts.len() == 1) as usize
    } else if springs.chars().nth(counts[0]) == Some('#') {
        0
    } else {
         validate_springs(&springs[counts[0]+1..], &counts[1..], cache)
    };
    cache.insert(cache_key, result);

    result
    
}

#[cfg(test)]
    mod tests {
    use super::*; 

    fn part2_wrapper(springs: &str, counts: &[usize]) -> usize {
        let mut cache: HashMap<(String,Vec<usize>), usize> = HashMap::new();
        let new_springs = springs.chars().chain(once('?')).cycle().take(5 * (springs.len() + 1) - 1).collect::<String>();
        let new_counts = counts.iter().cycle().take(counts.len() * 5).copied().collect::<Vec<_>>();
    
        validate_springs(&new_springs, &new_counts, &mut cache)
    }

    #[test]
    fn test_spring_combos() {
        let mut cache: HashMap<(String,Vec<usize>), usize> = HashMap::new();

        assert_eq!(validate_springs(".??..??...?##.", &[1,1,3], &mut cache), 4);
        assert_eq!(validate_springs("?#?#?#?#?#?#?#?", &[1,3,1,6], &mut cache),1);
        assert_eq!(validate_springs("????.#...#...", &[4,1,1], &mut cache),1);
        assert_eq!(validate_springs("????.######..#####.", &[1,6,5], &mut cache),4);
        assert_eq!(validate_springs("?###????????", &[3,2,1], &mut cache), 10);
    }

    #[test]
    fn test_spring_combos_p2() {
        assert_eq!(part2_wrapper("???.###", &[1,1,3]), 1);
        assert_eq!(part2_wrapper("??.?#?###???..#??", &[6,2]), 32);
        assert_eq!(part2_wrapper(".??..??...?##.", &[1,1,3]), 16384);
        assert_eq!(part2_wrapper("?#?#?#?#?#?#?#?", &[1,3,1,6]),1);
        assert_eq!(part2_wrapper("????.#...#...", &[4,1,1]),16);
        assert_eq!(part2_wrapper("????.######..#####.", &[1,6,5]),2500);
        assert_eq!(part2_wrapper("?###????????", &[3,2,1]),506250);
    }

    #[test]
    fn test_part1() {
        let data = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
    let plans = parse_data(data);
        assert_eq!(part1(&plans), 21);

    }


    #[test]
    fn test_part2() {
        let data = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
    let plans = parse_data(data);
        assert_eq!(part2(&plans), 525152);

    }
}
