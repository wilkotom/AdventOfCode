use std::error::Error;
use aochelpers::get_daily_input;

fn main() -> Result<(), Box<dyn Error>>{
    let dataset = get_daily_input(9,2023)?.split('\n')
        .map(|l| l.split(' ')
            .filter_map(|x| x.parse::<i64>().ok())
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();
    println!("Part 1 {}", dataset.iter().map(|l| extrapolate_last_value(l)).sum::<i64>());
    println!("Part 2 {}", dataset.iter().map(|l| extrapolate_last_value(&l.iter().rev().copied().collect::<Vec<_>>())).sum::<i64>());
    Ok(())
}

fn extrapolate_last_value(values: &[i64]) -> i64 {
    let differences = values[1..].iter().enumerate().map(|(i, x)| x - values[i]).collect::<Vec<_>>();
    if differences.iter().all(|x| x == &0) {
        *values.iter().last().unwrap()
    } else {
        values.iter().last().unwrap() + extrapolate_last_value(&differences)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extrapolator() {
        assert_eq!(extrapolate_last_value(&[0,3,6,9,12,15]), 18);
        assert_eq!(extrapolate_last_value(&[15,12,9,6,3,0]), -3);
        
        assert_eq!(extrapolate_last_value(&[1,3,6,10,15,21]), 28);
        assert_eq!(extrapolate_last_value(&[21,15,10,6,3,1]), 0);

        assert_eq!(extrapolate_last_value(&[10,13,16,21,30,45]), 68);
        assert_eq!(extrapolate_last_value(&[45,30,21,16,13,10]), 5);

    }
}