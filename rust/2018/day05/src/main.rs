#[derive(Debug,Copy,Clone)]
struct Unit {
    prev: Option<usize>,
    next: Option<usize>,
    polarity: bool,
    label: char
}

// Doubly linked inspired solve; way, way faster than string manipulation

fn main() {
    let poly_string = std::fs::read_to_string("./input.txt").unwrap();
    let mut polymer = Vec::new();
    // I add "anchors" at each end of the molecule which will never
    // react, so I always know that the unit at the start points to the first real unit,
    // which may not be the next indexed one in the vec.
    // units are retained in the vec to avoid the cost penalty of deleting them
    polymer.push(Unit { 
        prev: None,
        next: Some(1), 
        polarity: false,  
        label: '*'});
    
    for (i, c) in poly_string.chars().enumerate()  {
        let new_poly = Unit {
            prev: Some(i), 
            next: Some(i+2), 
            polarity: c.is_uppercase(),  
            label: c.to_lowercase().next().unwrap() };
        polymer.push(new_poly);
    }

    polymer.push(Unit { 
        prev: Some(poly_string.len() -1), 
        next: None, 
        polarity: false,  
        label: '*'});

    println!("Part 1: {}", collapse_poly(polymer.clone(), ' '));
    println!("Part 2: {}", part2(polymer));

}


fn collapse_poly(mut poly: Vec<Unit>, skip: char) -> usize {
    let mut ptr = 0;
    while poly.get(ptr).unwrap().next.is_some() {
        let cur = *poly.get(ptr).unwrap();
        let next = *poly.get(cur.next.unwrap()).unwrap();
        if cur.label == skip {
            let previous = cur.prev.unwrap();
            poly.get_mut(previous).unwrap().next = cur.next;
            poly.get_mut(cur.next.unwrap()).unwrap().prev = cur.prev;
            ptr = cur.prev.unwrap();
        } else if next.label == cur.label && next.polarity != cur.polarity {
            let previous = cur.prev.unwrap();
            let nextnext = next.next.unwrap();
            poly.get_mut(previous).unwrap().next = next.next;
            poly.get_mut(nextnext).unwrap().prev = cur.prev;
            if cur.prev.is_some() {
                ptr = previous;
            }
        } else {
            ptr = cur.next.unwrap();
         }
    }
    let mut ptr = poly.first().unwrap().next;
    let mut counter = 0;
    while ptr.is_some() {
        ptr = poly.get(ptr.unwrap()).unwrap().next;
        counter +=1;
    }
    // counter includes start anchor but not end.
    counter -1 

}

fn part2(poly: Vec<Unit>) -> usize {
    let mut best = poly.len();
    for c in 'a'..='z' {
        best = best.min(collapse_poly(poly.clone(), c));
    }
    best
}
