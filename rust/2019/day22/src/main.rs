use num_bigint::{BigInt, ToBigInt};
use num_traits::{Zero, One};
use std::fs::read_to_string;

fn main() {
    let instructions = read_to_string("./day22/input.txt").unwrap();
    println!("Part 1: {}", part1(10007, &instructions).iter().position(|x| *x == 2019).unwrap());
    println!("Part 2: {}", part2(119315717514047, &instructions));
}

fn part1(deck_len: usize, instructions: &str) -> Vec<usize> {
    let mut deck = (0..deck_len).collect::<Vec<_>>();
    
    for instruction in instructions.split('\n') {
        let mut words = instruction.split_ascii_whitespace();
        words.next();
        match words.next() {
            None => unimplemented!(),
            Some("with") => {
                words.next();
                let n = words.next().unwrap().parse::<usize>().unwrap();
                let new_deck = deck.clone();
                for i in 0..deck_len {
                    deck[i*n % deck_len] = new_deck[i];
                }
            },
            Some("into") => {
                deck.reverse();
            },
            Some(s) => {
                let mut n = s.parse::<i32>().unwrap();
                if n > (deck_len as i32) / 2 {
                    n -= deck_len as i32;
                } else if  n < -(deck_len as i32) / 2  {
                    n += deck_len as i32;
                }
                if n < 0 {
                    deck.rotate_right((-n) as usize);
                } else {
                    deck.rotate_left(n as usize);
                }
            }
        }
    }
    deck
}

fn part2(deck_len: i128, instructions: &str) -> BigInt {
    
    let (one_pass_factor, one_pass_increment) =  collapse_rules(deck_len, instructions);
    
    let num_passes: BigInt = 101741582076661_i64.to_bigint().unwrap();
    let deck_len = deck_len.to_bigint().unwrap();
    let final_factor = mod_pow(one_pass_factor.clone(), num_passes, &deck_len);
    let final_increment: BigInt = (one_pass_increment * (final_factor.clone() -1) * mod_pow(one_pass_factor -1 , &deck_len -2 , &deck_len)) % &deck_len;
    let position = 2020.to_bigint().unwrap();

    (((position - final_increment) * ( mod_pow(final_factor, &deck_len -2 , &deck_len)))  % &deck_len) + deck_len

    
}

fn collapse_rules(deck_len: i128, instructions: &str) -> (BigInt, BigInt) {
	/*
		Each rule can be reduced to a transformation:
		x => (ax +b) % deckSize

		Hence by combining the rules, the whole set can also be reduced to a single transformation

        "Deal into new stack" => "Deal with increment <deck size> -1 / cut -1"
	*/

    let mut a: BigInt = One::one();
    let mut b: BigInt = Zero::zero();



    for instruction in instructions.split('\n') {
        let mut da: BigInt = Zero::zero();
        let mut db: BigInt = Zero::zero();
        let mut words = instruction.split_ascii_whitespace();
        words.next();
        match words.next() {
            None => unimplemented!(),
            Some("with") => { //deal with increment
                words.next();
                da = words.next().unwrap().parse::<BigInt>().unwrap();
            },
            Some("into") => { // new stack
                da -= 1;
                db -= 1;
            },
            Some(s) => {  // "cut"
                da = One::one();
                db -= s.parse::<BigInt>().unwrap();
            }

        }
        a = (&da * a + deck_len) % deck_len;
        b=  (da *b + db + deck_len)  %deck_len;
    }

    

    (a,b)
}

fn mod_pow(mut base: BigInt, mut exp: BigInt, modulus: &BigInt) -> BigInt {
    if modulus == &One::one() { return Zero::zero() }
    let mut result: BigInt = One::one();
    base %=  modulus.clone();
    while exp.clone() > Zero::zero() {
        if exp.clone() % 2 == One::one() {
            result = result * base.clone() % modulus;
        }
        exp >>= 1;
        base = base.pow(2) % modulus;
    }
    result
}

#[test]
fn first_example() {
    let instructions = "deal with increment 7\ndeal into new stack\ndeal into new stack";
    let result = part1(10, instructions);
    assert_eq!(result, vec![0,3,6,9,2,5,8,1,4,7]);
}

#[test]
fn second_example() {
    let instructions = "cut 6\ndeal with increment 7\ndeal into new stack";
    let result = part1(10, instructions);
    assert_eq!(result, vec![3,0,7,4,1,8,5,2,9,6]);
}

#[test]
fn third_example() {
    let instructions = "deal into new stack\ncut -2\ndeal with increment 7\ncut 8\ncut -4\ndeal with increment 7\ncut 3\ndeal with increment 9\ndeal with increment 3\ncut -1";
    let result = part1(10, instructions);
    assert_eq!(result, vec![9,2,5,8,1,4,7,0,3,6]);
}