fn main() {
    let data = std::fs::read_to_string("./input.txt").unwrap();
    println!("{}", data.split('\n').map(is_nice_part1).filter(|x| *x).count());
    println!("{}", data.split('\n').map(is_nice_part2).filter(|x| *x).count());

}

fn is_nice_part1(word: &str) -> bool {
    let mut vowel_count =0;
    let mut doubles = false;
    for (i, c) in word.chars().enumerate() {
        match c {
            'a'|'e'|'i'|'o'|'u' => {vowel_count +=1},
            _ => {}
        }
        if i > 0 {
            match c {
                'b' => {
                    if word.chars().nth(i-1) == Some('a') {
                        return false;
                    }
                }
                'd' => {
                    if word.chars().nth(i-1) == Some('c') {
                        return false;
                    }
                }
                'q' => {
                    if word.chars().nth(i-1) == Some('p') {
                        return false;
                    }
                }
                'y' => {
                    if word.chars().nth(i-1) == Some('x') {
                        return false;
                    }
                }
                _ => {}
                
            }
            if word.chars().nth(i-1) == Some(c) {
                doubles = true
            }
        }
    }
    vowel_count >=3 && doubles
}


fn is_nice_part2(word: &str) -> bool {
    let mut oxo = false;
    let mut repeated = false;

    for i in 0..word.len()-2 {
        if word.chars().nth(i) == word.chars().nth(i+2) {
            oxo = true;
        }
        if word[i+2..].contains(&[word.chars().nth(i).unwrap_or('!'), word.chars().nth(i+1).unwrap_or('!')].iter().collect::<String>()) {
            repeated = true;    
        }
        if oxo && repeated {
            return true;
        }
    }

    
    false
}