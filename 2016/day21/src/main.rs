use itertools::Itertools;

fn main() {
    let starting_string= String::from("abcdefgh");
    let part2_string =   String::from("fbgdceah");
    let instructions = std::fs::read_to_string("./input.txt").unwrap();
    let ending_string = part1(&starting_string, &instructions);
    let combs = ending_string.chars().permutations(8);

    for perm in combs {
        if part1(&perm.iter().collect::<String>(), &instructions) == part2_string {
            println!("Moo {:?}", perm.iter().collect::<String>());
            break;
        }
    }

}



fn part1 (starting_string: &str, instructions: &str) -> String {
    let mut letters: Vec<_> = starting_string.chars().collect();
    // println!("{:?}", letters);
    
    for line in instructions.split('\n') {
        let words = line.split_ascii_whitespace().collect::<Vec<&str>>();
        
        match words.get(0) {
            Some(&"swap") => {
                match words.get(1) {
                    Some(&"position") => {
                        let a = words.get(2).unwrap().parse::<usize>().unwrap();
                        let b = words.get(5).unwrap().parse::<usize>().unwrap();
                        letters.swap(a, b);
                    },
                    Some(&"letter") => {
                        let a = letters.iter().position(|x| *x == words[2].chars().next().unwrap()).unwrap();
                        let b = letters.iter().position(|x| *x == words[5].chars().next().unwrap()).unwrap();
                        letters.swap(a, b);
                    },
                    _ => unreachable!()
                }
            },
            Some(&"rotate") => {
                match words.get(1) {
                    Some(&"right") => {
                        let c = words.get(2).unwrap().parse::<usize>().unwrap();
                        letters.rotate_right(c)
                    },
                    Some(&"left") => {
                        let c = words.get(2).unwrap().parse::<usize>().unwrap();
                        letters.rotate_left(c)
                    },
                    Some(&"based") => {
                        let mut c =  letters.iter().position(|x| *x == words[6].chars().next().unwrap()).unwrap();
                        if c >= 4 {
                            c += 2
                        } else {
                            c += 1
                        }
                        c %= letters.len();
                        letters.rotate_right(c)
                    },
                    _ => unreachable!()
                }

            },
            Some(&"move") => {
                let a = words.get(2).unwrap().parse::<usize>().unwrap();
                let b = words.get(5).unwrap().parse::<usize>().unwrap();
                let c = letters.remove(a);
                letters.insert(b, c);
            },
            Some(&"reverse") => {
                let mut a = words.get(2).unwrap().parse::<usize>().unwrap();
                let mut b = words.get(4).unwrap().parse::<usize>().unwrap();
                while b > a {
                    letters.swap(a, b);
                    a += 1;
                    b -= 1;
                }
            }
            _ => unreachable!() 
        };
        
    }
    letters.iter().collect::<String>()
}
