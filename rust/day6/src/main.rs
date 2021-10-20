use std::fs;
use std::io::Error;
use std::collections::HashSet;
use std::iter::FromIterator;


fn main() -> Result<(), Error> {
    let data = fs::read_to_string("../input.txt")?;
    let records: Vec<_> = data.split("\n\n").collect();
    let mut everyone_answered = 0;
    let mut anyone_answered = 0;
    for record in records.iter(){
        let mut any_answered: HashSet<char> = HashSet::new();
        let mut every_answered: HashSet<char> = HashSet::new();
        let passengers: Vec<_> = record.split("\n").collect();
        let mut first = true;
        for passenger in passengers.iter() {
            let mut passenger_answer: HashSet<char> = HashSet::new();
            for (_, question) in passenger.chars().enumerate() {
                any_answered.insert(question);
                passenger_answer.insert(question);
            }
            if every_answered.len() == 0 && first{
                for answer in passenger_answer {
                    every_answered.insert(answer);
                }
            } else {
                every_answered = HashSet::from_iter(every_answered.intersection(&passenger_answer).map(|x| *x));
            }
            first = false;
        }
        anyone_answered += any_answered.len();
        everyone_answered += every_answered.len();
    }
    println!("Anyone answered total: {}", anyone_answered);
    println!("Everyone answered total: {}", everyone_answered);

    Ok(())
}