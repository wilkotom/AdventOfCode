use std::{collections::HashMap, error::Error};
use aochelpers::{get_daily_input, Coordinate};

 
fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(21,2024)?;
    let sequences = data.lines().collect::<Vec<&str>>();
    println!("{}", part1(&sequences));
    println!("{}", part2(&sequences));

    Ok(())
}

fn part1(numbers: &[&str]) -> usize {
    numbers.iter().map(|s| shortest_sequence(&first_stage(s),1) * s[0..3].parse::<usize>().unwrap()).sum()
}

fn part2(numbers: &[&str]) -> usize {
    numbers.iter().map(|s: &&str| shortest_sequence(&first_stage(s),24) * s[0..3].parse::<usize>().unwrap()).sum()
}

fn shortest_sequence(sequence: &str, depth: usize) -> usize {
    let direction_lookups: HashMap<char, HashMap<char, &str>> = HashMap::from([
        ('A', HashMap::from([('A', "A"),    ('^', "<A"),  ('>', "vA"),  ('v', "<vA"), ('<', "v<<A")  ])),
        ('^', HashMap::from([('A', ">A"),   ('^', "A"),   ('>', "v>A"), ('v', "vA"),  ('<', "v<A")  ])),
        ('v', HashMap::from([('A', "^>A"),  ('^', "^A"),  ('>', ">A"),  ('v', "A"),   ('<', "<A")  ])),
        ('<', HashMap::from([('A', ">>^A"), ('^', ">^A"), ('>', ">>A"), ('v', ">A"),  ('<', "A")  ])),
        ('>', HashMap::from([('A', "^A"),   ('^', "<^A"), ('>', "A"),   ('v', "<A"),  ('<', "<<A")  ])),
    ]);

    let mut cache: HashMap<(char, char, usize), usize> = HashMap::new();
    for start in ['A', '<', 'v', '>', '^'] {
        for end in  ['A', '<', 'v', '>', '^'] {
            cache.insert((start, end, 0), direction_lookups.get(&start).unwrap().get(&end).unwrap().len());
        }
    }
    let mut current_button = 'A';
    let mut result = 0;
    for target in  sequence.chars() {
        result+= shortest_instruction_length(current_button, target, depth, &mut cache, &direction_lookups);
        current_button = target;
    }
    result
}


fn shortest_instruction_length(
            start:char, 
            end:char, 
            level: usize, 
            cache: &mut HashMap<(char, char, usize), usize>,
            direction_lookups: &HashMap<char, HashMap<char, &str>>) -> usize {

    if let Some(precomputed) = cache.get(&(start,end,level))  {
        return *precomputed;
    }
    let target = direction_lookups.get(&start).unwrap()
            .get(&end).unwrap();
    let mut result = 0;
    let mut current='A';
    for next_char in target.chars() {
        result += shortest_instruction_length(current, next_char, level-1, cache, direction_lookups);
        current = next_char;
    }
    cache.insert((start,end,level), result);
    result
}

fn first_stage(sequence: &str) -> String {
    let direction_lookups: HashMap<char, HashMap<char, &str>> = HashMap::from([
        ('A', HashMap::from([('A', ""), ('0', "<"),('1', "^<<"),('2', "<^"),('3', "^"),('4', "^^<<"),('5', "<^^"),('6', "^^"),('7', "^^^<<"),('8', "^^^<"), ('9', "^^^")])),
        ('0', HashMap::from([('A', ">"), ('0', ""),('1', "^<"),('2', "^"),('3', ">^"),('4', "^^<"),('5', "^^"),('6', ">^^"),('7', "^^^<"),('8', "^^^"), ('9', ">^^^")])),
        ('1', HashMap::from([('A', ">>v"), ('0', ">v"),('1', ""),('2', ">"),('3', ">>"),('4', "^"),('5', "^>"),('6', ">>^"),('7', "^^"),('8', "^^>"), ('9', "^^>>")])),
        ('2', HashMap::from([('A', ">v"), ('0', "v"),('1', "<"),('2', ""),('3', ">"),('4', "^<"),('5', "^"),('6', ">^"),('7', "<^^"),('8', "^^"), ('9', ">^^")])),
        ('3', HashMap::from([('A', "v"), ('0', "v<"),('1', "<<"),('2', "<"),('3', ""),('4', "^<<"),('5', "^<"),('6', "^"),('7', "<<^^"),('8', "^^<"), ('9', "^^")])),
        ('4', HashMap::from([('A', ">>vv"), ('0', ">vv"),('1', "v"),('2', ">v"),('3', ">>v"),('4', ""),('5', ">"),('6', ">>"),('7', "^"),('8', ">^"), ('9', ">>^")])),
        ('5', HashMap::from([('A', ">vv"), ('0', "vv"),('1', "v<"),('2', "v"),('3', ">v"),('4', "<"),('5', ""),('6', ">"),('7', "^<"),('8', "^"), ('9', "^>")])),
        ('6', HashMap::from([('A', "vv"), ('0', "<vv"),('1', "<<v"),('2', "<v"),('3', "v"),('4', "<<"),('5', "<"),('6', ""),('7', "<<^"),('8', "^<"), ('9', "^")])),
        ('7', HashMap::from([('A', ">>vvv"), ('0', ">vvv"),('1', "vv"),('2', "vv>"),('3', ">>vv"),('4', "v"),('5', ">v"),('6', ">>v"),('7', ""),('8', ">"), ('9', ">>")])),
        ('8', HashMap::from([('A', ">vvv"), ('0', "vvv"),('1', "vv<"),('2', "vv"),('3', "vv>"),('4', "v<"),('5', "v"),('6', ">v"),('7', "<"),('8', ""), ('9', ">")])),
        ('9', HashMap::from([('A', "vvv"), ('0', "vvv<"),('1', "v<<"),('2', "vv<"),('3', "vv"),('4', "<<v"),('5', "<v"), ('6', "v"),('7', "<<"),('8', "<"), ('9', "")])),
    ]);
    let mut current_button = 'A';
    let mut result = String::new();
    for target in  sequence.chars() {
        result.push_str(direction_lookups.get(&current_button).unwrap().get(&target).unwrap());
        result.push('A');
        current_button = target;
    }

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    fn validate_final_instructions(sequence: &str) -> String {
        let keypad = HashMap::from([
            (Coordinate{x:0,y:0}, '7'),
            (Coordinate{x:1,y:0}, '8'),
            (Coordinate{x:2,y:0}, '9'),
            (Coordinate{x:0,y:1}, '4'),
            (Coordinate{x:1,y:1}, '5'),
            (Coordinate{x:2,y:1}, '6'),
            (Coordinate{x:0,y:2}, '1'),
            (Coordinate{x:1,y:2}, '2'),
            (Coordinate{x:2,y:2}, '3'),
            (Coordinate{x:1,y:3}, '0'),
            (Coordinate{x:2,y:3}, 'A'),
        ]);
    
        let mut pos = Coordinate{x:2, y:3};
        let mut res = String::new();
        for c in sequence.chars() {
            match c {
                '^' => {pos.y -=1},
                'v' => {pos.y +=1}
                '<' => {pos.x -=1}
                '>' => {pos.x +=1}
                'A' => {res.push(*keypad.get(&pos).unwrap()); }
                _ => unimplemented!()
            }
        }
        res
    }

    fn validate_intermediate_instructions(sequence: &str) -> String {
        let keypad = HashMap::from([
            (Coordinate{x:1,y:0}, '^'),
            (Coordinate{x:2,y:0}, 'A'),
            (Coordinate{x:0,y:1}, '<'),
            (Coordinate{x:1,y:1}, 'v'),
            (Coordinate{x:2,y:1}, '>'),
        ]);
        let mut pos = Coordinate{x:2, y:0};
        let mut res = String::new();
        for c in sequence.chars() {
            match c {
                '^' => {pos.y -=1},
                'v' => {pos.y +=1}
                '<' => {pos.x -=1}
                '>' => {pos.x +=1}
                'A' => {res.push(*keypad.get(&pos).unwrap()); }
                _ => unimplemented!()
            }
        }
        res
    }
    
    #[test]
    fn test_part_1_029() {
        let first_pass = first_stage("029A");
        assert_eq!(validate_final_instructions(&first_pass), "029A");
    }

    #[test]
    fn test_part_1_980() {
        let first_pass = first_stage("980A");
        assert_eq!(validate_final_instructions(&first_pass), "980A");
    }


    #[test]
    fn test_part_1_179() {
        let first_pass = first_stage("179A");
        assert_eq!(validate_final_instructions(&first_pass), "179A");
    }


    #[test]
    fn test_part_1_456() {
        let first_pass = first_stage("456A");
        assert_eq!(validate_final_instructions(&first_pass), "456A");
    }


    #[test]
    fn test_part_1_379() {
        let first_pass = first_stage("379A");
        assert_eq!(validate_final_instructions(&first_pass), "379A");
    }

    #[test]
    fn test_shortest_seq_p1() {
        assert_eq!(shortest_sequence(&first_stage("029A"),1), 68);
        assert_eq!(shortest_sequence(&first_stage("980A"),1), 60);
        assert_eq!(shortest_sequence(&first_stage("179A"),1), 68);
        assert_eq!(shortest_sequence(&first_stage("456A"),1), 64);
        assert_eq!(shortest_sequence(&first_stage("379A"),1), 64);
    }
    #[test]
    fn test_calculate_compound_complexity() {
        assert_eq!(part1(&["029A", "980A", "179A", "456A", "379A"]), 126384);
    }


    #[test]
    fn test_part2() {
    }

}
/*
  left: "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>Av<<A>A>^AAAvA<^A>A"
 right: "v<<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A"
      */
