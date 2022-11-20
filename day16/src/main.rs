use std::collections::VecDeque;

const INPUT: &str = "10011111011011001";
const SIZE: usize = 35651584;

fn main() {
    let data = dragon_curve(String::from(INPUT), SIZE);
    // println!("{}", data);
    let checksum = checksum(data);
    println!("{}", checksum);


}

fn dragon_curve(mut text: String, desired: usize) -> String {
    while text.len() < desired {
        let invert = text.clone().as_bytes().iter().rev().map(|x| if *x == 49 {'0'} else {'1'}).collect::<String>();
        text.push('0');
        text.push_str(&*invert)
    }
    text.truncate(desired);
    text

}

fn checksum(text: String) -> String {
    let mut q = text.into_bytes().iter().cloned().collect::<VecDeque<_>>();
    while q.len() % 2 == 0{
        let mut next = VecDeque::new();
        while q.len() > 0 {
            if q.pop_front() == q.pop_front() {
                next.push_back('1' as u8)
            } else {
                next.push_back('0' as u8)
            }
        }
        q.append(&mut next)
    }
    let mut result = String::new();
    while q.len() > 0{
        result.push(q.pop_front().unwrap() as char);
    }
    result
}