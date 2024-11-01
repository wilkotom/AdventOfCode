use std::fs::read_to_string;
use hashbrown::HashSet;

fn main() {
    let input = read_to_string("./input.txt").unwrap();
    let mut total = 0;
    for passphrase in input.split('\n') {
        let vec_words = passphrase.split_ascii_whitespace().collect::<Vec<_>>();
        let set_words = passphrase.split_ascii_whitespace().collect::<HashSet<_>>();
        if vec_words.len() == set_words.len() {
            total += 1;
        }
    }
    println!("{}", total);

    let mut total = 0;
    for passphrase in input.split('\n') {
        let mut vec_words = Vec::new();
        let mut set_words = HashSet::new();
        for word in passphrase.split_ascii_whitespace() {
            let mut w = word.to_owned().chars().collect::<Vec<_>>();
            w.sort_unstable();
            set_words.insert(w.to_owned());
            vec_words.push(w);
        }
        if vec_words.len() == set_words.len() {
            total += 1;
        }
    }
    println!("{}", total);
}
