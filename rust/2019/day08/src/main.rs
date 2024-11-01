fn main() {
    let width = 25;
    let height = 6;
    let data = std::fs::read_to_string("./day08/input.txt").unwrap();
    let mut min_zeroes = usize::MAX;
    let mut ones_count = 0;
    let mut twos_count = 0;
    let mut layers = Vec::new();
    for i in (0..data.len()).step_by(width*height) {
        layers.push(data[i..i+width*height].to_owned());
        if data[i..i+width*height].matches('0').count() < min_zeroes {
            min_zeroes = data[i..i+width*height].matches('0').count();
            ones_count = data[i..i+width*height].matches('1').count();
            twos_count = data[i..i+width*height].matches('2').count();
        }
    }
    println!("Part 1: {}", ones_count * twos_count);
    layers.reverse();
    let res = merge_layers(layers);
    for (i,c) in res.chars().enumerate() {
        if c == '0' {
            print!(" ");
        } else {
            print!("#");
        }
        if (i+1) %25 == 0 {
            println!();
        }
    }
}

fn merge_layers(mut layers: Vec<String>) -> String {
    if layers.is_empty() {
        String::new()
    }
    else if layers.len() == 1 {
        layers.pop().unwrap()
    } else {
        let top_layer = layers.pop().unwrap();
        let next_layer = layers.pop().unwrap();
        let mut new_top = String::new();
        for (i, c) in top_layer.chars().enumerate() {
            match c {
                '0' | '1' => new_top.push(c),
                _ => new_top.push(next_layer.chars().nth(i).unwrap())
            };
        }
        layers.push(new_top);
        merge_layers(layers)
    }
}