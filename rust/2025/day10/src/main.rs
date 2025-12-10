use std::collections::VecDeque;
use aochelpers::get_daily_input;
use std::{fmt, error::Error};
use good_lp::*;

#[derive(Debug, Clone)]
struct Machine{
    desired_state: u32,
    buttons: Vec<u32>,
    raw_buttons: Vec<Vec<usize>>,
    joltages:Vec<u32>
}

impl fmt::Display for Machine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({:?}), {{{:?}}}", self.render_state(self.desired_state),  self.raw_buttons, self.joltages)
    }
}

impl Machine {
    fn render_state(&self, state: u32) -> String {
        let column_count = self.buttons.iter().map(|b|b.ilog2()).max().unwrap() +1;
        let mut lights = String::new();
        for i in 0..column_count {
            if state & 2_u32.pow(i as u32) == 2_u32.pow(i as u32) {
                lights.push_str("#");
            } else {
                lights.push_str(".");
            }
        }
        format!("[{}]", lights.chars().collect::<String>())
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let data = get_daily_input(10, 2025)?;
    let machines = parse_data(&data);
    println!("Part 1: {}", part1(&machines));
    println!("Part 2: {}", part2(&machines));

    Ok(())
}

fn part1(machines: &[Machine]) -> usize {
    machines.iter().map(|m: &Machine| minimum_presses_part1(m)).sum()
}

fn part2(machines: &[Machine]) -> usize {
    machines.iter().map(|m: &Machine| minimum_presses_part2(m)).sum()
}


fn minimum_presses_part1(machine: &Machine) -> usize {
    let mut next_presses: VecDeque<(usize, u32)> = VecDeque::new();
     for button in machine.buttons.iter() {
        next_presses.push_back((1, *button ));
    }
    while let Some ((presses, state)) =  next_presses.pop_front() {
        if state == machine.desired_state {
            return presses;
        }
        for button in machine.buttons.iter() {
            next_presses.push_back((presses+1, state ^ button ));
        }
    }
    0
}

fn minimum_presses_part2(machine: &Machine) -> usize {

    let mut vars = ProblemVariables::new();

    // Press Counts represents the number of times each button is pressed
    // Without specifying a floor, the minimum number of presses is
    // negative infinity.
    let mut button_presses = Vec::new();
    for _ in 0..machine.raw_buttons.len() {
        // register each button's press count as a variable with the solver
        let variable = vars.add(variable().min(0).integer());
        // keep track of the variables so we can state the objective in terms 
        // of them below
        button_presses.push(variable);
    }

    // We state the problem: smallest sum of all button presses
    let mut problem = good_lp::highs(vars.minimise(button_presses.iter().sum::<Expression>()));

    // Available solvers listed at https://docs.rs/crate/good_lp/latest
    // Use of the `highs`` solver as it:
    // - Requires no extra libraries (ruling out coin_cbc)
    // - Supports integers (clarabel doesn't support these, despite the 
    //   docstring, highs does)
    // - Gives the right answer (sorry microlp)
    // - Is "fast"
    // - Doesn't require any additional mucking about to get it working
    //   (most of the others)

    // The value of each joltage counter is derived from the buttons pressed 
    // we have one expression per counter
    let mut expressions = vec![Expression::with_capacity(machine.raw_buttons.len()); machine.joltages.len()];

    for i in 0..machine.raw_buttons.len() {
        for x in machine.raw_buttons[i].iter() {
            // for each button pressed, add the number of times it is pressed
            // to the total for the joltage counters it increments
            expressions[*x] += button_presses[i];
        }
    }
    for (e, j) in expressions.into_iter().zip(machine.joltages.clone()) {
        // for each of the expressions for a given joltage counter's value, 
        // add the constraint that the result of the expression must be the desired 
        // joltage
        problem.add_constraint(e.eq(j as f64));
    }
    let solution = problem.solve().unwrap();
    button_presses.iter().map(|&v| solution.value(v)).sum::<f64>() as usize

}


fn parse_data(data: &str) -> Vec<Machine> {
    let mut machines = Vec::new();
    for line in data.lines() {
        let mut desired_state = 0;
        let mut buttons = Vec::new();
        let mut raw_buttons = Vec::new();
        let mut joltages = Vec::new();
        for section in line.split_ascii_whitespace() {
            match section.chars().nth(0) {
                Some('[') => {
                    for c in section[1..section.len() -1].chars().rev() {

                        match c {
                            '#' => {
                                desired_state <<= 1;
                                desired_state |= 1;
                            }
                            '.' => {
                                desired_state <<=1;
                            }
                            _ => unimplemented!()
                        }
                    }

                },
                Some('(') => {
                    let mut press = 0;
                    let mut original_presses = Vec::new();
                    for button in section[1..section.len() -1].split(',') {
                        press |= 2_u32.pow(button.parse().unwrap());
                        original_presses.push(button.parse::<usize>().unwrap());
                    }
                    buttons.push(press);
                    raw_buttons.push(original_presses);
                },
                Some('{') => {
                    for joltage in section[1..section.len() -1].split(',') {
                        joltages.push(joltage.parse().unwrap());
                    }
                },
                _ => unreachable!()
            }
        }
        machines.push(Machine{ desired_state, buttons, raw_buttons, joltages});
    }

    machines
}


#[cfg(test)]
mod tests {
    const TESTDATA: &str= "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
    use super::*;
    
    #[test]
    fn test_parser() {
        let machines = parse_data(TESTDATA);
        for machine in machines {
            println!("{}", machine);
        }
    }

    #[test]
    fn test_min_presses_p1() {
        let machines = parse_data(TESTDATA);
        let expected_results = vec![2,3,2];
        for (machine,result) in machines.iter().zip(expected_results) {
            assert_eq!(minimum_presses_part1(machine), result);
        }
    }

    #[test]
    fn test_min_presses_p2() {
        let machines = parse_data(TESTDATA);
        let expected_results = vec![10,12,11];
        for (machine,result) in machines.iter().zip(expected_results) {
            assert_eq!(minimum_presses_part2(machine), result);
        }
    }

    #[test]
    fn test_p1() {
        let machines = parse_data(TESTDATA);
        assert_eq!(part1(&machines), 7);
    }

    #[test]
    fn test_p2() {
        let machines = parse_data(TESTDATA);
        assert_eq!(part2(&machines), 33);
    }

}