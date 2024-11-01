fn main() {
    let input = String::from("yzbqklnj");
    let mut number = 1;
    loop {
        let checksum = format!("{:x}", md5::compute(format!("{}{}", input, number)));
        if &checksum[0..6] == "000000" {
            break
        }
        number +=1;

    }

    println!("{}", number);
}
