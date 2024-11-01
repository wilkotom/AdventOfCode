fn main() {
    let input_string = "1113222113";
    let mut input = input_string.chars().collect::<Vec<_>>();
    for _ in 0..50 {
        input = iterate(&input);
        println!("{:?}", input.len());
    }
}

fn iterate(input: &[char]) -> Vec<char> {
    let mut output = Vec::new();
    if !input.is_empty() {
        let mut last = input[0];
        let mut count:u8 = 1;
        for c in input[1..].iter() {
            if *c == last {
                count +=1;
            } else {
                output.push( (count + 48) as char);
                output.push( last);
                count = 1;
                last = *c;
            }
        }
        output.push((count + 48) as char);
        output.push(last);
    }
    output
}
