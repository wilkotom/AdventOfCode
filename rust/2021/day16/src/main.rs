use std::{str::Chars, iter::Peekable};

struct PacketResult {
    version_total: i64,
    result: i64
}

fn main() {
    let data = hex_to_binary(&std::fs::read_to_string("./input.txt").unwrap());
    let mut packet = data.chars().peekable();
    let result = parse_packet(&mut packet);
    println!("Part 1 {}", result.version_total);
    println!("Part 2 {}", result.result);
}


fn parse_packet(packet: &mut Peekable<Chars>) -> PacketResult {
    let mut version_total = bits_to_val(packet, 3);
    let type_id = bits_to_val(packet, 3);
    let result = if type_id == 4 { // literal packet
        let mut result = 0;
        let mut last_nybble = false;
        while !last_nybble {
                let next_chunk = bits_to_val(packet, 5);
                result <<= 4;
                result |= next_chunk & !16;
                last_nybble = (next_chunk & 16) == 0;
        }
        result
    } else { // operator packet
        let mut values:Vec<i64> = Vec::new();
        if bits_to_val(packet, 1) == 0 {
            // next 15 bits are total length in bits of sub-packet
            let packet_bit_count = bits_to_val(packet, 15) as usize;
            let next_chars = next_n_bit_string(packet, packet_bit_count);
            let mut packet_bits = next_chars.chars().peekable();
            while packet_bits.peek().is_some() {
                let next_packet = parse_packet(&mut packet_bits);
                values.push(next_packet.result);
                version_total += next_packet.version_total;
            }
        } else {
            // next 11 bits are total number of sub-packets
            let mut sub_packet_count = bits_to_val(packet, 11);
            while sub_packet_count > 0 {
                let next_packet = parse_packet(packet);
                values.push(next_packet.result);
                version_total += next_packet.version_total;
                sub_packet_count -=1
            }
        }
        match type_id {
            0 => values.iter().sum::<i64>(),
            1 => values.iter().product::<i64>(),
            2 => *values.iter().min().unwrap(),
            3 => *values.iter().max().unwrap(),
            5 => if values[0] > values[1] {1} else {0}
            6 => if values[0] < values[1] {1} else {0}
            7 => if values[0] == values[1] {1} else {0}
            _ => unreachable!()
        }
    };
    PacketResult{version_total, result}
}

fn hex_to_binary(hex: &str) -> String {
    let mut result: String = String::new();
    for c in hex.chars() {
        let n = i32::from_str_radix(&c.to_string(), 16).unwrap();
        result.push_str(&format!("{:04b}", n));
    }

    result
}

fn bits_to_val(chars: &mut Peekable<Chars>, mut n: usize) -> i64{
    let mut result = 0;
    while n > 0 {
        result <<= 1;
        result += if chars.next().unwrap() == '1' {1} else {0};
        n -= 1
    }
    result
}

fn next_n_bit_string(chars: &mut Peekable<Chars>, mut n: usize) -> String{
    // When we recurse, we need to pass a Peekable<Char> of a certain length to treat
    // as if it were the whole. Building a String and then deconstricting it again is 
    // a nasty hack to avoid lifetime issues with slices.
    let mut result = String::new();
    while n > 0 {
        result.push(chars.next().unwrap());
        n -= 1
    }
    result
}