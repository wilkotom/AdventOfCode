fn main() {
    let data = std::fs::read_to_string("./input.txt").unwrap();
    let mut program = data.split('\n').map(|x| x.parse::<isize>().unwrap()).collect::<Vec<_>>();

    println!("{:?}", program);

    let mut pc: isize = 0;
    let mut steps = 0;

    while (pc as usize) < program.len() {
        steps += 1;
        // println!("{} {} {:?}", steps, pc, program);
        let step = program[pc as usize];
        if step >=3 {
            program[pc as usize] -=1;
        } else {
            program[pc as usize] +=1;
        }
        pc += step ;
    }
    println!("{:?}", steps);
}
