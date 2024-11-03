fn main() {
    let mut programs = "abcdefghijklmnop".chars().collect::<Vec<_>>();

    let data = std::fs::read_to_string("./input.txt").unwrap();
    println!("{}", programs.iter().collect::<String>() );
    dance(&mut programs, &data);

    println!("Part 1: {}", programs.iter().collect::<String>() );
    let mut counter = 1;
    while programs.iter().collect::<String>() != *"abcdefghijklmnop" {
        dance(&mut programs, &data);
        counter +=1;
    }
    
    programs = "abcdefghijklmnop".chars().collect::<Vec<_>>();

    for _ in  0..(1_000_000_000 % counter) {
        dance(&mut programs, &data);
    }
    println!("Part 2: {}" , programs.iter().collect::<String>())
}

fn dance(programs: &mut [char], instructions: &str) {
    for dance_move in instructions.split(',') {
        match dance_move.chars().next() {
            Some('s') => {
                programs.rotate_right(dance_move[1..].parse::<usize>().unwrap());
            },
            Some('x') => {
                let mut program_names = dance_move[1..].split('/');
                let one = program_names.next().unwrap().parse::<usize>().unwrap();
                let two = program_names.next().unwrap().parse::<usize>().unwrap();
                programs.swap(one, two);

            },
            Some('p') => {
                let one = programs.iter().position(|&x| x == dance_move.chars().nth(1).unwrap()).unwrap();
                let two = programs.iter().position(|&x| x == dance_move.chars().nth(3).unwrap()).unwrap();
                programs.swap(one,two);
            },
            _ => unimplemented!()
        }
        
    }
}