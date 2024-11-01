use std::fs::read_to_string;
use itertools::Itertools;

fn main() {
    let data = read_to_string("./input.txt").unwrap();
    let packages = data.split('\n').map(|x| x.parse::<isize>().unwrap()).collect::<Vec<_>>();
    let target = packages.iter().sum::<isize>() / 4;
    println!("{}", get_target_groups(target, packages, 4));

}

fn get_target_groups(target: isize, numbers: Vec<isize>, groups: i32) -> isize{
    let mut min_entanglement = isize::MAX;
    if groups == 1 {
        if numbers.iter().sum::<isize>() == target {
            1
        } else {
            0
        }
    } else {
        for comb_count in 1..numbers.len() {
            for  combination in numbers.iter().combinations(comb_count) {
                if combination.iter().copied().sum::<isize>() == target{
                    let remaining = numbers.iter().filter(|n| !combination.contains(n)).copied().collect::<Vec<_>>();
                    let target_groups = get_target_groups(target, remaining, groups -1);
                    if target_groups != 0 {
                        let entanglement:isize  = combination.iter().copied().product();
                        min_entanglement = min_entanglement.min(entanglement);
                        if target_groups == 1 {
                            return min_entanglement;
                        }
                    }
                }
            }
            if min_entanglement != isize::MAX {
                return min_entanglement;
            }
        }
        0
    }
}