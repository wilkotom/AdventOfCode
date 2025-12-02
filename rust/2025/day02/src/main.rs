use aochelpers::get_daily_input;
use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(2,2025)?;
    let ranges = parse_data(&data);
    println!("Part 1: {}", part1(&ranges));
    println!("Part 2: {}", part2(&ranges));
    Ok(())
}

fn part1(ranges: &[(usize,usize)]) -> usize{
    ranges.iter().map(|r| repeated_once(r)).sum()
}

fn repeated_once(range: &(usize,usize)) -> usize {
    let mut res: usize = 0;
    let digit_counts = ((range.0.ilog10() +1 ) as usize, (range.1.ilog10() +1) as usize);
    if digit_counts.0 %2 == 0 { // an even number of digits for first half
        let f1 = range.0 / 10_usize.pow((digit_counts.0 / 2).try_into().unwrap());
        let f2 = range.1 / 10_usize.pow((digit_counts.1 /2).try_into().unwrap());
        for i in f1..=f2 {
            let candidate = (i * 10_usize.pow((digit_counts.0 / 2).try_into().unwrap()))+ i;
            if candidate >= range.0 && candidate <= range.1 && candidate.ilog(10) % 2 == 1{
                res +=candidate;
            }
        }
    } else if digit_counts.1 %2 == 0 { 
        let f1 = 10_usize.pow((digit_counts.0 /2).try_into().unwrap());
        let f2: usize = range.1 / 10_usize.pow((digit_counts.1 /2).try_into().unwrap());
        for i in f1..=f2 {
            let candidate = (i * 10_usize.pow((digit_counts.0 / 2 +1).try_into().unwrap()))+ i;
            if candidate >= range.0 && candidate <= range.1 && candidate.ilog(10) % 2 == 1{
                res +=candidate;
            }
        }
    }
    res
}

fn part2(ranges: &[(usize,usize)]) -> usize {
    ranges.iter().map(|r| repeats(*r)).sum()
}

fn repeats(range: (usize,usize)) -> usize {
    let digit_counts: (usize, usize) = ((range.0.ilog10() +1 ) as usize, (range.1.ilog10() +1) as usize);
    let mut match_sum = 0;
    'outer: for num in range.0..=range.1 {
        for digit_count in digit_counts.0..=digit_counts.1 {
            for repeat_len in 1..= digit_count/2 {
                if digit_count % repeat_len != 0 {
                    continue;
                }
                let first_n_digits = num / 10_usize.pow((digit_count - repeat_len) as u32);
                if repeated_digits(num, first_n_digits) {
                    match_sum += num;
                    continue 'outer;
                }
            }
        }
    }
    match_sum
}

fn repeated_digits(num: usize, c: usize) -> bool {
    if c == 0 {
        return false;
    }
    let last_digits = num % 10_usize.pow(c.ilog10() as u32 +1);
    if num == 0 {
        true
    } else if last_digits != c{
        false
    } else {
        repeated_digits(num / 10_usize.pow(c.ilog10() as u32 +1), c)
    }
}

fn parse_data(data: &str) -> Vec<(usize, usize)> {
    data.split(',').map(|e| (e.split('-').next().unwrap().parse::<usize>().unwrap(), e.split('-').last().unwrap().parse::<usize>().unwrap())).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const P1TESTDATA: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_p1(){
        let data = parse_data(P1TESTDATA);
        assert_eq!(part1(&data), 1227775554);
    }
        #[test]
    fn test_p2(){
        let data = parse_data(P1TESTDATA);
        assert_eq!(part2(&data), 4174379265);
    }

    #[test]
    fn test_repeated_digits(){
        assert!(repeated_digits(11, 1));
        assert!(repeated_digits(121212, 12));
        assert!(repeated_digits(123123, 123));
        assert!(repeated_digits(12341234, 1234));
        assert!(!repeated_digits(12341234, 123));

        assert!(!repeated_digits(121212, 122));
        assert!(!repeated_digits(121212, 1));


    }
}