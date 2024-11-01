
fn main() {
    let mut password = "cqjxxyzz".chars().collect::<Vec<_>>();
    increment(&mut password);
    while !is_valid(&password) {
        increment(&mut password);
    }
    println!("{}", password.iter().collect::<String>() ) ;

}

fn increment(password: &mut Vec<char>) {
    let mut pos = password.len() -1 ;
    let mut flipped = true;
    while flipped {
        flipped = false;
        password[pos] = (password[pos] as u8 +1) as char;
        if password[pos] == '{' {
            password[pos] = 'a';
            if pos > 0 {
                flipped = true;
                pos -=1;
            } else {
                password.insert(0, 'a');
            }
        }
        
    }
}

fn is_valid(password: &[char]) -> bool{
    let mut ascending = false;
    let mut blacklisted = false;
    let mut repeated: Option<char> = None;
    let mut two_repeats = false;
    for (i, c) in password.iter().enumerate() {
        
        if i < password.len() -2 {
            ascending |= password[i] as u8 +1 == password[i+1] as u8 && password[i] as u8 +2 == password[i+2] as u8; 
        }
        if i < password.len() -1 {
            if password[i] == password[i+1] && repeated == None {
                repeated = Some(password[i]);
            } else if password[i] == password[i+1] {
                two_repeats |= Some(password[i]) != repeated;
            }
        }
        blacklisted |= matches!(c, 'i' | 'o' | 'l')
    }
    ascending && two_repeats && !blacklisted
}
