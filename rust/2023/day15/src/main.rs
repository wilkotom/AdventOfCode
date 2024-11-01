use std::error::Error;
use aochelpers::get_daily_input;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Lens {
    label: String,
    value: usize
}

fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(15,2023)?;
    println!("Part 1: {}", data.split(',').map(checksum).sum::<usize>());
    println!("Part 2: {}", part2(&data));
    Ok(())
}

fn part2(data: &str) -> usize {
    let mut hm: [Vec<Lens>; 256]  = std::array::from_fn(|_| vec![]);
    for step in data.split(',') {
        if step.contains('-') {
            hm[checksum(&step[..step.len()-1])].retain(|e| e.label != step[..step.len()-1]);
        }
        else if step.contains('=') {
            let label = &step[..step.chars().position(|c| c == '=').unwrap()];
            let value = step[step.chars().position(|c| c == '=').unwrap() +1 ..].parse::<usize>().unwrap();
            let cs = checksum(label);
            let mut found = false;
            hm[cs].iter_mut().for_each(|l| if l.label == label {l.value = value; found = true} );
            if ! found {
                hm[cs].push(Lens{label: label.to_string(), value})
            }
        }
    }
    hm.iter().enumerate().map(
        |(box_number, lens_list)| lens_list.iter().enumerate().map(
            |(i, lens)|  (box_number+1) * (i +1) * lens.value).sum::<usize>()
        ).sum()
}

fn checksum(data: &str) -> usize {
    data.chars().fold(0, |a, c | ((a+ c as usize) * 17) % 256)
}


#[cfg(test)]
mod tests {

    use super::*; 


    #[test]
    fn test_checksum() {
        assert_eq!(checksum("HASH"), 52);
        assert_eq!("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".split(',').map(checksum).sum::<usize>(), 1320)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"), 145)
    }
}
