fn main() {
    let input = String::from("reyedfim");
    let mut number = 0;
    let mut part_1_password = String::new();
    let mut part_2_password: Vec<Option<char>> = vec![None; 8];
    while part_1_password.len() < 8 || part_2_password.contains(&None) {
        let checksum = format!("{:x}", md5::compute(format!("{}{}", input, number)));
        if &checksum[0..5] == "00000" {
            if part_1_password.len() < 8 {
                let password_char = checksum.chars().nth(5).unwrap();
                part_1_password.push(password_char);
                println!("{} {}", number, part_1_password);

            }
            if part_2_password.contains(&None) {
                let position  = checksum.chars().nth(5).unwrap().to_string().parse::<usize>().unwrap_or(9);
                let password_char = checksum.chars().nth(6).unwrap();
                if position < 8 && part_2_password[position].is_none() {
                    part_2_password[position] = Some(password_char);
                }
                println!("{} {}", number, part_2_password.iter().map(|x| x.unwrap_or('_')).collect::<String>());
            }


        }
        number += 1;
    }
    println!("Password is {}", part_1_password);
    println!("Part 2 Password is {}", part_2_password.iter().map(|x| x.unwrap_or('_')).collect::<String>());

}
