use aochelpers::get_daily_input;
use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(12, 2025)?;
    let (_, regions) = parse_data(&data);
    println!("Part 1: {:?}", part1(&regions));
    Ok(())
}

fn part1(regions: &[((usize,usize), Vec<usize>)]) -> usize {
    let mut answer = 0;
    for (region, present_counts) in regions {
        let three_by_three_squares = region.0 / 3 * region.1 / 3;
        // Naive solution: All the presents fit in a 3x3 grid 
        // For my input we don't need to pack them tightly.
        if three_by_three_squares >= present_counts.iter().sum() {
            answer +=1;
        }
    }
    answer
}

fn parse_data(data: &str) -> (Vec<usize>, Vec<((usize, usize), Vec<usize>)> ){

    let mut shape_sizes = Vec::new();
    let mut regions = Vec::new();
    for section in data.split("\n\n") {
        if section.contains('#') {
            shape_sizes.push(section.chars().filter(|&c| c == '#').count())
        } else {
            for line in section.lines() {
                let mut entries = line.split_ascii_whitespace();
                let dimensions = entries.next().unwrap();
                let x = dimensions[..dimensions.find('x').unwrap()].parse::<usize>().unwrap();
                let y = dimensions[dimensions.find('x').unwrap() +1..dimensions.len() -1].parse::<usize>().unwrap();
                let counts = entries.map(|e| e.parse().unwrap()).collect();
                regions.push(((x,y), counts));
            }

        }
    }


    (shape_sizes, regions)

}


#[cfg(test)]
mod tests {

    
}