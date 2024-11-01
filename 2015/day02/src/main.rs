fn main() {
    let data = std::fs::read_to_string("./input.txt").unwrap();
    let mut paper = 0;
    let mut ribbon = 0;
    for line in data.split('\n') {
        let mut dims = line.split('x');
        let x = dims.next().unwrap().parse::<i32>().unwrap();
        let y = dims.next().unwrap().parse::<i32>().unwrap();
        let z = dims.next().unwrap().parse::<i32>().unwrap();
        paper += x*y*2 + x*z*2 + y*z *2 + (x*y).min((x*z).min(y*z));
        ribbon += (2*(x+y)).min((2*(x+z)).min(2*(y+z))) + x*y*z;

        // println!("{:?} {}", (x,y,z),x*y*2 + x*z*2 + y*z *2 + x*y.min(x*z.min(y*z)));

    }
    println!("{}", paper);
    println!("{}", ribbon);

}
