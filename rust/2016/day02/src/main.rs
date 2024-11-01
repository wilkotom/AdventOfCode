use std::collections::HashMap;
#[derive(Debug, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down
}

fn main() {
    let key_layout = HashMap::from([
        (1, HashMap::from([
            (Direction::Down, 4),
            (Direction::Right, 2)
        ])),
        (2, HashMap::from([
            (Direction::Down, 5),
            (Direction::Left, 1),
            (Direction::Right, 3)

        ])),
        (3, HashMap::from([
            (Direction::Down, 6),
            (Direction::Left, 2)
        ])),
        (4, HashMap::from([
            (Direction::Up, 1),
            (Direction::Down, 7),
            (Direction::Right, 5)
        ])),
        (5, HashMap::from([
            (Direction::Up, 2),
            (Direction::Right, 6),
            (Direction::Down, 7),
            (Direction::Left, 4)
        ])),
        (6, HashMap::from([
            (Direction::Up, 3),
            (Direction::Down, 9),
            (Direction::Left, 5)
        ])),
        (7, HashMap::from([
            (Direction::Up, 4),
            (Direction::Right, 8)
        ])),
        (8, HashMap::from([
            (Direction::Up, 5),
            (Direction::Right, 9),
            (Direction::Left, 7)
        ])),
        (9, HashMap::from([
            (Direction::Up, 6),
            (Direction::Left, 8)
        ]))
    ]);
    let key_layout_part_2 = HashMap::from([
        (1, HashMap::from([
            (Direction::Down, 3),
            (Direction::Right, 2)
        ])),
        (2, HashMap::from([
            (Direction::Down, 6),
            (Direction::Right, 3)

        ])),
        (3, HashMap::from([
            (Direction::Up, 1),
            (Direction::Down, 7),
            (Direction::Left, 2),
            (Direction::Right, 4)
        ])),
        (4, HashMap::from([
            (Direction::Down, 8),
            (Direction::Left, 3)
        ])),
        (5, HashMap::from([
            (Direction::Right, 6),
        ])),
        (6, HashMap::from([
            (Direction::Up, 2),
            (Direction::Down, 10),
            (Direction::Left, 5),
            (Direction::Right, 7)
        ])),
        (7, HashMap::from([
            (Direction::Up, 3),
            (Direction::Down, 11),
            (Direction::Left, 6),
            (Direction::Right, 8)
        ])),
        (8, HashMap::from([
            (Direction::Up, 4),
            (Direction::Down, 12),
            (Direction::Right, 9),
            (Direction::Left, 7)
        ])),
        (9, HashMap::from([
            (Direction::Left, 8)
        ])),
        (10, HashMap::from([
            (Direction::Up, 6),
            (Direction::Right, 11)
        ])),
        (11, HashMap::from([
            (Direction::Up, 7),
            (Direction::Down, 13),
            (Direction::Left, 10),
            (Direction::Right, 12)
        ])),
        (12, HashMap::from([
            (Direction::Up, 8),
            (Direction::Left, 11)
        ])),
        (13, HashMap::from([
            (Direction::Up, 11)
        ]))
    ]);

    let instructions = std::fs::read_to_string(String::from("./input.txt")).unwrap().split("\n").map(|x| String::from(x)).collect::<Vec<_>>();
    println!("{:?}", key_layout);
    let mut part_1_keypresses: Vec<i32> = Vec::new();
    let mut part_2_keypresses: Vec<i32> = Vec::new();

    let mut part1_key = 5;
    let mut part2_key = 5;
    for instruction in instructions{
        for c in instruction.chars() {
            part1_key = match c {
                'U' => *key_layout.get(&part1_key).unwrap().get(&Direction::Up).unwrap_or(&part1_key),
                'D' => *key_layout.get(&part1_key).unwrap().get(&Direction::Down).unwrap_or(&part1_key),
                'L' => *key_layout.get(&part1_key).unwrap().get(&Direction::Left).unwrap_or(&part1_key),
                'R' => *key_layout.get(&part1_key).unwrap().get(&Direction::Right).unwrap_or(&part1_key),
                _ => 0
            };
            part2_key = match c {
                'U' => *key_layout_part_2.get(&part2_key).unwrap().get(&Direction::Up).unwrap_or(&part2_key),
                'D' => *key_layout_part_2.get(&part2_key).unwrap().get(&Direction::Down).unwrap_or(&part2_key),
                'L' => *key_layout_part_2.get(&part2_key).unwrap().get(&Direction::Left).unwrap_or(&part2_key),
                'R' => *key_layout_part_2.get(&part2_key).unwrap().get(&Direction::Right).unwrap_or(&part2_key),
                _ => 0
            };
        }
        part_1_keypresses.push(part1_key);
        part_2_keypresses.push(part2_key);

    }
    println!("{}", part_1_keypresses.iter().map(|x| format!("{}", x)).collect::<Vec<_>>().join(""));
    println!("{}", part_2_keypresses.iter().map(|x| format!("{:X}", x)).collect::<Vec<_>>().join(""));

}
