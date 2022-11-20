use std::cmp::Ordering;

fn main() {
    let lower = 256310;
    let upper = 732736;

    let mut part1_counter = 0;
    let mut part2_counter = 0;

    for number in lower..upper +1 {
        part1_counter += if part1(number) {1} else {0};
        part2_counter += if part2(number) {1} else {0};

    }
    println!("Part 1: {}", part1_counter);
    println!("Part 2: {}", part2_counter);
}


fn part1(mut n: i32) -> bool {
    let mut ascending = true;
    let mut two_together = false;
    while n > 0 {
        let last = n % 10;
        n /= 10;
        let next = n % 10;
        if last < next {
            ascending = false;
        }
        if last == next {
            two_together = true;
        }
    }
    ascending && two_together
}

fn part2(mut n: i32) -> bool {
    let mut ascending = true;
    let mut last = i32::MAX;
    let mut two_together= false;
    let mut count = 0;
    while n > 0 {
        match (n%10).cmp(&last) {
            Ordering::Less => {
                if count == 1{
                    two_together = true
                }
                count = 0;
            },
            Ordering::Equal => {count +=1},
            Ordering::Greater => {
                ascending = false;
                break;
            },
        }
        last = n %10;
        n /= 10;
    }
    
    two_together |= count ==1;
    ascending && two_together


}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_111111() {
        assert!(part1(111111));
    }

    #[test]
    fn test_223450() {
        assert!(!part1(223450));
    }

    #[test]
    fn test_123789() {
        assert!(!part1(123789));
    }

    #[test]
    fn test_112233() {
        assert!(part2(112233));
    }

    #[test]
    fn test_123444() {
        assert!(!part2(123444));
    }

    #[test]
    fn test_111122() {
        assert!(part2(111122));
    }

    #[test]
    fn test_113444() {
        assert!(part2(113444));
    }
}
