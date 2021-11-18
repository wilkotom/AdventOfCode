fn main() {
    let card_pub_key :i64 = 5764801;
    let door_pub_key :i64 = 17807724;

    let mut card_loop_size = 1;

    let subject_number = 7;
    let mut card_val = subject_number;

    while card_val != card_pub_key {
        card_val = (card_val * subject_number) % 20201227;
        card_loop_size +=1;
    }

    let mut secret_encryption_key = 1;
    for _ in 0..card_loop_size {
        secret_encryption_key = (secret_encryption_key * door_pub_key) % 20201227;
    }
    println!("Encryption key is: {}", secret_encryption_key);
}
