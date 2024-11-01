#[derive(Debug,Clone)]
struct Pair {
    left: PairValue,
    right: PairValue
}

#[derive(Debug,Clone)]
enum PairValue {
    SingleValue(u32),
    NestedPair(Box<Pair>)
}

fn print_pair(p: &Pair) {
    print!("[");
    match &p.left {
        PairValue::SingleValue(x) => {
            print!("{}", x);
        }
        PairValue::NestedPair(p) => {
            print_pair(&p);
        }
    }
    print!(",");
    match &p.right {
        PairValue::SingleValue(x) => {
            print!("{}", x);
        }
        PairValue::NestedPair(p) => {
            print_pair(&p);
        }
    }
    print!("]");
}

fn main() {
    let data = std::fs::read_to_string("./input.txt").unwrap();
    let mut sums: Vec<PairValue> = Vec::new();
    let mut dangling: Vec<PairValue> = Vec::new();
    for line in data.split('\n') {
        for char in line.chars() {
            match char {
                '[' | ',' => {},
                ']' => {
                    let right = dangling.pop().unwrap();
                    let left = dangling.pop().unwrap();
                    let new_pair  = Pair{left, right};
                    dangling.push(PairValue::NestedPair(Box::new(new_pair)));

                },
                c => {dangling.push(PairValue::SingleValue(c.to_digit(10).unwrap()))}
            }
        }
        let x = dangling.pop().unwrap();
        sums.push(x);
        
    }
    let mut processed: Vec<Pair> = Vec::new();
    for s in sums {
        if let PairValue::NestedPair(p) = s {
            processed.push(process_line(p));
        }
    }

    while processed.len() >= 2 {
        let left = PairValue::NestedPair(Box::new(processed.remove(0)));
        let right = PairValue::NestedPair(Box::new(processed.remove(0)));
        let new_pair = Box::new(Pair{left,right});
        let completed = process_line(new_pair);
        processed.insert( 0, completed);
    };
    let final_sum = processed.pop().unwrap();
    println!("Part 1: Score: {}", score(final_sum));

    let mut max_score = 0;
    let sum_strings = data.split('\n').collect::<Vec<_>>();
    for left_line in &sum_strings{

        for right_line in &sum_strings {
            for char in left_line.chars() {
                match char {
                    '[' | ',' => {},
                    ']' => {
                        let right = dangling.pop().unwrap();
                        let left = dangling.pop().unwrap();
                        let new_pair  = Pair{left, right};
                        dangling.push(PairValue::NestedPair(Box::new(new_pair)));
        
                    },
                    c => {dangling.push(PairValue::SingleValue(c.to_digit(10).unwrap()))}
                    }
            }
            let left_sum = dangling.pop().unwrap();
    
            for char in right_line.chars() {
                match char {
                    '[' | ',' => {},
                    ']' => {
                        let right = dangling.pop().unwrap();
                        let left = dangling.pop().unwrap();
                        let new_pair  = Pair{left, right};
                        dangling.push(PairValue::NestedPair(Box::new(new_pair)));
        
                    },
                    c => {dangling.push(PairValue::SingleValue(c.to_digit(10).unwrap()))}
                    }
            }
            let right_sum = dangling.pop().unwrap();
            let new_pair = Box::new(Pair{left: left_sum,right: right_sum});
            let completed = process_line(new_pair);
            max_score = max_score.max(score(completed));
        }
    }

    println!("Part 2: {}", max_score);
}

fn score(p: Pair) -> u32 {
    match p { 
        Pair{left: PairValue::SingleValue(l), right: PairValue::SingleValue(r)} => {3 * l + 2 * r},
        Pair{left: PairValue::NestedPair(l), right: PairValue::SingleValue(r)} => {3 * score(*l) + 2 * r},
        Pair{left: PairValue::SingleValue(l), right: PairValue::NestedPair(r)} => {3*l + 2* score(*r)},
        Pair{left: PairValue::NestedPair(l), right: PairValue::NestedPair(r)} => {3*score(*l) + 2* score(*r)}
    }
}

fn process_line(mut pair: Box<Pair>) -> Pair{
    
    let mut finished = false;
    while !finished {
        finished = true;
        let mut exploded = true;
        while exploded {
            let res = explode(*pair, 0);
            exploded = res.1;
            pair = Box::new(res.0);
        }

        let res = split_pair(*pair);
        pair = Box::new(res.0);
        if res.1 {
            finished = false;
        }

    }

    *pair
}

fn split_pair(p: Pair) -> (Pair,bool) {
    let (left, left_changed) = split_pairvalue(p.left);
    if left_changed {
        (Pair{left, right: p.right}, true)
    } else {
        let (right, right_changed) = split_pairvalue(p.right);
        (Pair{left, right}, right_changed)
    }
}

fn split_pairvalue(p: PairValue) -> (PairValue, bool) {
    match p {
        PairValue::SingleValue(v) =>{ 
            if v > 9 {
                let left = PairValue::SingleValue( v /2);
                let right = PairValue::SingleValue(v - (v/2));
                let new_pair = Pair{left,right};
                (PairValue::NestedPair(Box::new(new_pair)), true)
            } else {
                (p, false)
            }
        }
        PairValue::NestedPair(p) => {
            let left_leaf = p.left;
            let right_leaf = p.right;
            let (new_left, left_split) = split_pairvalue(left_leaf);
            if left_split {
                let new_nested = Pair{left:new_left,right: right_leaf};
                (PairValue::NestedPair(Box::new(new_nested)), true)
            } else {
                let (new_right, right_split) = split_pairvalue(right_leaf);
                let new_nested = Pair{left:new_left,right: new_right};
                (PairValue::NestedPair(Box::new(new_nested)), right_split)
            }
        }
    }
}

fn explode(pair: Pair, depth: i32) -> (Pair, bool, u32, u32) {
    if depth == 3 {
        match pair { 
            Pair{ left: PairValue::NestedPair(l),  right: PairValue::SingleValue(r) } => {
                let inner_left = match l.left {
                    PairValue::SingleValue(v) => v,
                    _ => unimplemented!()
                };
                let inner_right = match l.right {
                    PairValue::SingleValue(v) => v,
                    _ => unimplemented!()
                };
                let new_right = inner_right + r;
                let new_pair = Pair{left:PairValue::SingleValue(0), right: PairValue::SingleValue(new_right)};
                (new_pair, true, inner_left, 0)
            },
            Pair{left: PairValue::SingleValue(l), right: PairValue::NestedPair(r) } => {
                let inner_left = match r.left {
                    PairValue::SingleValue(v) => v,
                    _ => unimplemented!()
                };
                let inner_right = match r.right {
                    PairValue::SingleValue(v) => v,
                    _ => unimplemented!()
                };
                let new_left = inner_left + l;
                let new_pair = Pair{left:PairValue::SingleValue(new_left), right: PairValue::SingleValue(0)};
                (new_pair, true, 0, inner_right)
            },
            Pair{left: PairValue::NestedPair(l), right: PairValue::NestedPair(r)} => {
                let inner_left = match l.left {
                    PairValue::SingleValue(v) => v,
                    _ => unimplemented!()
                };
                let inner_right = match l.right {
                    PairValue::SingleValue(v) => v,
                    _ => unimplemented!()
                };
                let new_pair = Pair{left:PairValue::SingleValue(0), right: PairValue::NestedPair(Box::new(increment_leftmost(*r, inner_right)))};
                (new_pair, true, inner_left, 0)
            },
            Pair{left: PairValue::SingleValue(_),
                right: PairValue::SingleValue(_)
            } => {
                (pair, false, 0, 0)
            }
        }
    } else {
        match pair { 
            Pair{left: PairValue::SingleValue(_), right: PairValue::SingleValue(_) } => {
                (pair, false, 0, 0)
            },
            Pair{ left: PairValue::NestedPair(l), right: PairValue::SingleValue(r)  } => {
                let (new_left, result, left_bubble, right_bubble) = explode(*l, depth+1);
                (Pair{left: PairValue::NestedPair(Box::new(new_left)), right: PairValue::SingleValue(r+right_bubble)}, result, left_bubble, 0)
            },
            Pair{left: PairValue::SingleValue(l), right: PairValue::NestedPair(r) } => {

                let (new_right, result, left_bubble, right_bubble)= explode(*r, depth+1);
                (Pair{left: PairValue::SingleValue(l+left_bubble), right: PairValue::NestedPair(Box::new(new_right))}, result, 0, right_bubble)

            },
            Pair{left: PairValue::NestedPair(l), right: PairValue::NestedPair(r), } => {
                let (new_left, result, left_bubble, right_bubble) = explode(*l, depth+1);
                if result {
                    (Pair{left: PairValue::NestedPair(Box::new(new_left)), 
                          right: PairValue::NestedPair(Box::new(increment_leftmost(*r, right_bubble)))},
                          result, left_bubble, 0)
                } else {
                    let (new_right, result, left_bubble, right_bubble)= explode(*r, depth+1);
                    (Pair{right: PairValue::NestedPair(Box::new(new_right)), 
                          left: PairValue::NestedPair(Box::new(increment_rightmost(new_left, left_bubble)))},
                          result, 0, right_bubble)
                }

            }
        }
    }
}


fn increment_leftmost(pair: Pair, val: u32) -> Pair {
    match pair { 
        Pair{left: PairValue::SingleValue(l),
            right: PairValue::SingleValue(r)
        } => {
            Pair{left: PairValue::SingleValue(l + val), right: PairValue::SingleValue(r)}
        },
        Pair{
            left: PairValue::NestedPair(l), 
            right: PairValue::SingleValue(r) 
        } => { Pair{left:  PairValue::NestedPair(Box::from(increment_leftmost(*l, val))), right: PairValue::SingleValue(r)}},
        Pair{
            left: PairValue::SingleValue(l), 
            right: PairValue::NestedPair(r), 
        } => { 
            Pair{left: PairValue::SingleValue(l + val), right: PairValue::NestedPair(r)}
        },
        Pair{
            left: PairValue::NestedPair(l), 
            right: PairValue::NestedPair(r), 
        } => {Pair{left:  PairValue::NestedPair(Box::from(increment_leftmost(*l, val))), right: PairValue::NestedPair(r)}}
    }

}

fn increment_rightmost(pair: Pair, val: u32) -> Pair {
    match pair { 
        Pair{left: PairValue::SingleValue(l),
            right: PairValue::SingleValue(r)
        } => {
            Pair{left: PairValue::SingleValue(l), right: PairValue::SingleValue(r + val)}
        },
        Pair{
            left: PairValue::NestedPair(l), 
            right: PairValue::SingleValue(r) 
        } => { Pair{left:  PairValue::NestedPair(l), right: PairValue::SingleValue(r + val)}},
        Pair{
            left: PairValue::SingleValue(l), 
            right: PairValue::NestedPair(r), 
        } => { 
            Pair{left: PairValue::SingleValue(l), right: PairValue::NestedPair(Box::new(increment_rightmost(*r, val)))}
        },
        Pair{
            left: PairValue::NestedPair(l), 
            right: PairValue::NestedPair(r), 
        } => {Pair{left:  PairValue::NestedPair(l), right: PairValue::NestedPair(Box::new(increment_rightmost(*r, val)))}}
    }

}