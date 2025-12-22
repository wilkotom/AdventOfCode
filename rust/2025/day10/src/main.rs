use std::{collections::VecDeque, isize,time::Instant};
use aochelpers::{get_daily_input, gcd};
use good_lp::{Expression, ProblemVariables, Solution, SolverModel, variable};
use z3::{SatResult, ast::Int};
use std::{fmt, error::Error};

#[derive(Debug, Clone)]
struct Machine{
    desired_state: i32,
    buttons: Vec<i32>,
    raw_buttons: Vec<Vec<usize>>,
    joltages:Vec<i32>
}


impl fmt::Display for Machine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({:?}), {{{:?}}}", self.render_state(self.desired_state),  self.raw_buttons, self.joltages)
    }
}

impl Machine {
    fn render_state(&self, state: i32) -> String {
        let column_count = self.buttons.iter().map(|b|b.ilog2()).max().unwrap() +1;
        let mut lights = String::new();
        for i in 0..column_count {
            if state & 2_i32.pow(i as u32) == 2_i32.pow(i as u32) {
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
    let now = Instant::now();
    println!("Part 2 (LP): {}   Elapsed: {:?}", part2_lp(&machines), now.elapsed());
    let now = Instant::now();
    println!("Part 2 (Z3): {}   Elapsed: {:?}", part2_z3(&machines), now.elapsed());
    let now = Instant::now();
    println!("Part 2 (Gauss): {}   Elapsed: {:?}", part2_gauss(&machines), now.elapsed());

    Ok(())
}

fn part1(machines: &[Machine]) -> usize {
    machines.iter().map(|m: &Machine| minimum_presses_part1(m)).sum()
}

fn part2_lp(machines: &[Machine]) -> isize {
    machines.iter().map(|m: &Machine| minimum_presses_part2_lp(m)).sum()
}

fn part2_z3(machines: &[Machine]) -> isize {
    machines.iter().map(|m: &Machine| minimum_presses_part2_z3(m)).sum()
}

fn part2_gauss(machines: &[Machine]) -> isize {
    machines.iter().map(|m: &Machine| minimum_presses_part2_gaussian(m)).sum()
}

fn minimum_presses_part1(machine: &Machine) -> usize {
    let mut next_presses: VecDeque<(usize, i32)> = VecDeque::new();
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

fn minimum_presses_part2_gaussian(machine: &Machine) -> isize {
    // The constraint-solver based solutions may be faster than this,
    // but this is the result of several days of plugging away at
    // linear algebra solutions and gaussian reduction
    // It's ours, precious, we understands its magic...
    // The slowness is probably due to the ridiculous number of 
    // allocations we do for potential partial and complete solutions.
    let mut matrix = Vec::new();
    for (i, joltage) in machine.joltages.iter().enumerate() {
        let mut row = Vec::new();
        for button in machine.raw_buttons.iter() {
            row.push (if button.contains(&i) { 1_isize} else {0})
        }
        row.push(*joltage as isize);
        matrix.push(row);
    }
    matrix = matrix_to_echelon_form(matrix);
    recursively_find_values(matrix).unwrap_or_default()
}

fn recursively_find_values(matrix: Vec<Vec<isize>>) -> Option<isize> {
    let last_val_position  = matrix[0].len() -1;
    
    let mut minima: Vec<Option<isize>> = vec![None; last_val_position];
    let mut maxima: Vec<Option<isize>> = vec![None; last_val_position];
    let mut known_values: Vec<_> = Vec::new();

    for (i, line) in matrix.iter().enumerate() {
        let target = line[last_val_position];
        let mut line_solved = true;
        for (j, v) in line[..last_val_position].iter().enumerate().skip(i+1).filter(|(_,v)| **v !=0) {
            line_solved = false;
            let needed_presses = target / v;
            if  target > 0 && needed_presses > 0 {
                if let Some(col_max) = maxima[j] {
                    maxima[j] = Some(col_max.max(needed_presses));
                } else {
                    maxima[j] = Some(needed_presses)
                }
            } else if target < 0 && needed_presses > 0 {
                if let Some(minimum) = minima[j] {
                    minima[j] = Some(minimum.min(needed_presses));
                } else {
                    minima[j] = Some(needed_presses);
                }
            }
        }
        if line_solved {
            if line[last_val_position] < 0 { 
                // Solution would require a negative number of button presses
                return None;
            } else if line[i] != 0 {
                if line[last_val_position] % line[i] != 0 {
                    // Solution would require a non-integer number of button presses
                    return None;
                }
                known_values.push(line[last_val_position] / line[i]);
            } 
        } 
    }

    if known_values.len() == matrix.len() {    
        return Some(known_values.iter().sum())
    }

    let mut answer = None;
    for (i, v) in maxima.iter().enumerate().filter(|(_,v)| v.is_some()).map(|(i,v)| (i, v.unwrap())) {
        for candidate_value in minima[i].unwrap_or_default()..= v {
            let mut candidate_matrix = matrix.clone();
            for line in candidate_matrix.iter_mut() {
                let multiple = line[i];
                line[last_val_position] -= candidate_value * multiple;
                line[i] = 0;
            }
            candidate_matrix[i][i] = 1;
            candidate_matrix[i][last_val_position] = candidate_value;
            if let Some(possible_best) = recursively_find_values(candidate_matrix) {
                if let Some(previous) = answer {
                    answer = Some(possible_best.min(previous))
                } else {
                    answer = Some(possible_best)
                }
            }
        }
    }

    answer
}

fn matrix_to_echelon_form(mut matrix: Vec<Vec<isize>>) -> Vec<Vec<isize>> {
    let mut pivot = 0;
    // Transform matrix into echelon form
    // first stage: ensure each row has a unique leading value

    let mut new_matrix = Vec::new();
    while !matrix.is_empty() && pivot <matrix[0].len() -1 {
        if let Some(row) = matrix.iter().position(|r| r[pivot] != 0) {
            let pivot_row: Vec<isize> = matrix.remove(row);
            for row in matrix.iter_mut(){
                if row[pivot] != 0 {
                    if row[pivot] % pivot_row[pivot] != 0 {
                        for item in row.iter_mut() {
                            *item *= pivot_row[pivot] ;
                        }
                    }
                }
                let multiplier = row[pivot] / pivot_row[pivot];
                for (j, item) in row.iter_mut().enumerate() {
                    *item -= pivot_row[j] * multiplier;
                }
            }
            new_matrix.push(pivot_row);
        }
        pivot +=1;

    }
    matrix = new_matrix;

    // pad the matrix so that matrix[i][i] is always a leading value
    for i in 0..matrix[0].len() -1 {
        if i >= matrix.len() || matrix[i][i] == 0 {
            matrix.insert(i, vec![0; matrix[0].len()]);
        }
    }

    // Now Reduce as far as possible :
    // remove multiples of each leading entry from the lines before it

    for pivot in 0..matrix.len(){
        let pivot_row = matrix[pivot].to_vec();
        if pivot_row[pivot] == 0 {
            continue;
        }
        for  (i, row) in matrix.iter_mut().enumerate() {
            if i >= pivot || row[pivot] == 0{
                continue;
            }
            if  row[pivot] % pivot_row[pivot] != 0 {
                for item in row.iter_mut() {
                    *item *= pivot_row[pivot] ;
                }
            }
            let multiplier = row[pivot] / pivot_row[pivot];
            for (j, item) in row.iter_mut().enumerate() {
                *item -= pivot_row[j] * multiplier;
            }
        }

    }

    for line in matrix.iter_mut() {
        if let Some(idx) = line.iter().position(|&v| v !=0) {
            let sign = line[idx].signum();
            let lcm = line[idx+1..].iter().filter(|&&v| v !=0 ).fold(line[idx], |l:isize ,&m| gcd(l.abs(), m.abs()));
            for entry in line.iter_mut() {
                *entry /=  lcm * sign;
                
            }
        }
    }
    matrix

}

fn minimum_presses_part2_z3 (machine: &Machine) -> isize {

    let solver = z3::Optimize::new();
    let mut button_presses = Vec::new();
    for i in 0..machine.raw_buttons.len() {
        let press_count = Int::fresh_const(&((i as u8 +65) as char).to_string() );
        solver.assert(&press_count.ge(0));
        button_presses.push(press_count);
    }

    for (i, &joltage) in machine.joltages.iter().enumerate() {
        let mut components = Vec::new();
        for (j, button) in machine.raw_buttons.iter().enumerate() {
            if button.contains(&i) {
                components.push(&button_presses[j]);
            }
        }
        solver.assert(&(Int::add(&components).eq(Int::from_u64(joltage as u64))));
    }

    let answer = Int::fresh_const("answer");
    solver.assert(&answer.eq(Int::add(&button_presses)));
    solver.minimize(&answer);

    match solver.check(&[]) {
        SatResult::Sat => solver
            .get_model()
            .unwrap()
            .eval(&answer, true)
            .and_then(|t| t.as_i64())
            .unwrap() as isize,
        _ => panic!("No solution found"),
    }
       
}

fn minimum_presses_part2_lp(machine: &Machine) -> isize {

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
    button_presses.iter().map(|&v| solution.value(v)).sum::<f64>() as isize

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
                    buttons.push(press as i32);
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
            assert_eq!(minimum_presses_part2_z3(machine), result);
            assert_eq!(minimum_presses_part2_lp(machine), result);
            assert_eq!(minimum_presses_part2_gaussian(machine), result);
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
        assert_eq!(part2_lp(&machines), 33);
    }
    
    #[test]
    fn test_non_integer_buttons () {
        let machines = parse_data("[.##.##] (4,5) (0,5) (2,3) (1,3,5) (0,3,5) (0,2,3,5) (0,1,4) (0,2,4,5) {198,181,22,50,173,65}");
        assert_eq!(minimum_presses_part2_gaussian(&machines[0]), 223);
        
    }

    #[test]
    fn test_negative_indeterminate_with_negative_total () {
        let machines = parse_data("[#...##] (0,2,4,5) (0,4,5) (0,1,2,4) (4,5) (0,1,3,5) (0,1) (3,4,5) (2,4) {48,28,10,28,40,57}");
        assert_eq!(minimum_presses_part2_gaussian(&machines[0]), 63);

    }
    
    #[test]
    fn test_no_single_unknown_in_any_line() {
        let machines = parse_data("[.#.#.#] (0,2,4) (0,1,2,5) (4) (0,2,3,5) (0,1,2,3) (1,2,4) (0,1,4,5) (1) {44,44,53,20,186,19}");
        assert_eq!(minimum_presses_part2_gaussian(&machines[0]), 207);
    }
    #[test]
    fn test_overcounting() {
        let machines = parse_data("[##...###.] (0,4,6,7) (0,1,2,3,5,8) (1,2,4,5,7,8) (1,3,4,5,7) (1,5) (0,1,5,6,7) (3,6) (0,1,3,5,6,7,8) (1,2,5,6,7) (3,4) (2) {22,51,40,24,41,51,42,54,24}");
        assert_eq!(minimum_presses_part2_gaussian(&machines[0]), 80);
    }

    #[test]
    fn test_only_negative_factors() {
        let machines = parse_data("[..##......] (0,2,5,6,9) (0,2,3,4,5,6,8,9) (0,1,4,5,7,8) (0,2,3,5,6,7,8) (1,7) (0,1,2,3,4,6,7,8) (0,1,3,4,6,7,8,9) (5,7,8,9) (0,1,3,4,5,6,7,9) (2,3,4,5,6,8,9) (0,1,3,7,8,9) (2,4,7) {230,50,217,218,66,218,212,244,232,60}
");
        assert_eq!(minimum_presses_part2_gaussian(&machines[0]), 266);
    }
    
    #[test]
    fn test_echelon_form() {
        assert_eq!(matrix_to_echelon_form(vec![vec![2,1,-1,8], vec![-3, -1, 2,-11], vec![-2,1,2,-3]]),
    vec![vec![1,0,0,2], vec![0,1,0,3], vec![0,0,1,-1]]);
    }


}