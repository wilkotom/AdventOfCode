use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Coordinate {
    x: usize,
    y: usize
}

fn main() {
    let file = std::fs::read_to_string(String::from("./input.txt")).unwrap();
    let instructions: Vec<_> = file.split("\n").map(|x| String::from(x)).collect();
    let mut display: HashMap<Coordinate,bool> = HashMap::new();
    for x in 0..50 {
        for y in 0..6 {
            display.insert(Coordinate{x, y}, false);
        }
    }
    execute_instructions(&mut display, instructions);
    let lit_pixels = display.values().filter(|&&x| x == true).collect::<Vec<_>>().len();
    println!("Lit pixels: {}", lit_pixels);
}

fn execute_instructions(display: &mut HashMap<Coordinate,bool>, instructions: Vec<String>) {
    for instruction in instructions {
        let tokens: Vec<_> = instruction.split(" ").collect();
        match tokens[0] {
            "rect" => {
                let size: Vec<_> = tokens[1].split("x").map(|x| x. parse::<usize>().unwrap()).collect();
                let max_x = size[0];
                let max_y = size[1];
                for x in 0.. max_x {
                    for y in 0..max_y {
                        display.insert(Coordinate { x, y }, true);
                    }
                }

            },
            "rotate" => {
                let dist = tokens[4].parse::<i32>().unwrap();
                let selector = *&tokens[2][2..].parse::<usize>().unwrap();
                match tokens[1] {
                    "row" => {
                        let mut row: Vec<bool> = Vec::new();
                        for x in 0..50 as usize{
                            let y = selector;
                            row.push( *display.get(&Coordinate{x,y}).unwrap());
                        }
                        for _ in 0..dist {
                            let t = row.pop().unwrap();
                            row.insert(0, t);
                        }
                        for x  in 0..50 as usize{
                            let y = selector;
                            display.insert(Coordinate{x ,y}, row[x]);
                        }

                    },
                    "column" => {
                        let mut col: Vec<bool> = Vec::new();
                        for y in 0..6 as usize{
                            let x = selector;
                            col.push( *display.get(&Coordinate{x,y}).unwrap());
                        }
                        for _ in 0..dist {
                            let t = col.pop().unwrap();
                            col.insert(0, t);
                        }
                        for y  in 0..6 as usize{
                            let x = selector;
                            display.insert(Coordinate{x ,y}, col[y]);
                        }
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }
    print_display(&display);
}

fn print_display(display: &HashMap<Coordinate,bool>) {
    for y in 0..6 {
        for x in 0..50 {
            print!("{}", if *display.get(&Coordinate{x,y}).unwrap_or(&false) {"#"} else {" "});
        }
        println!();
    }
}