fn main() {

    // bottom=left corner is squares of odd numbers

    let sought = 361527;
    let mut ring = 0;

    while i32::pow(ring *2 +1, 2)  < sought {
        ring +=1;
    }

    let br = i32::pow(ring *2 +1, 2);
    let tl = (i32::pow((ring-1) *2 +1, 2) + br) /2;
    let bl = (tl + br) / 2;
    let tr = tl - (br - bl);


    let ring_steps;
    if sought < tr {
        ring_steps = (((i32::pow((ring-1) *2 +1, 2) + tr) / 2) - sought).abs();
    } else if sought < tl {
        ring_steps = (((tl + tr) / 2) - sought).abs();
    } else if sought < bl {
        ring_steps = (((bl + tl) / 2) - sought).abs();
    } else {
        ring_steps = (((br + bl) / 2) - sought).abs();
    }

    println!("Answer: {}", ring + ring_steps);

}
