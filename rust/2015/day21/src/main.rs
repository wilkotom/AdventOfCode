use std::{cmp::Ordering, collections::BinaryHeap};
use itertools::Itertools;

#[derive(Clone,Debug, PartialEq, Eq, Hash)]
struct Object {
    name: String,
    // object_type: Type,
    cost: i32,
    damage: i32, 
    armour: i32
}

#[derive(Clone,Debug, PartialEq, Eq, Hash)]
struct Equipment {
    weapon: Object,
    armour: Object,
    left_ring: Object,
    right_ring: Object
}

impl Equipment {
    fn total_cost(&self) -> i32{
        self.armour.cost + self.weapon.cost + self.left_ring.cost + self.right_ring.cost
    }
    fn hit_value(&self) -> i32{
        self.armour.damage + self.weapon.damage + self.left_ring.damage + self.right_ring.damage
    }
    fn armour_value(&self) -> i32{
        self.armour.armour + self.weapon.armour + self.left_ring.armour + self.right_ring.armour
    }
}


impl Ord for Equipment {
    fn cmp(&self, other: &Self) -> Ordering {
        other.total_cost().cmp(&self.total_cost())
    }
}

impl PartialOrd for Equipment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


fn main() {
    let weapons = [Object{name: "Dagger".to_owned(), cost: 8, damage: 4, armour: 0},
        Object{name: "Shortsword".to_owned(), cost: 10, damage: 5, armour: 0},
        Object{name: "Warhammer".to_owned(), cost: 25, damage: 6, armour: 0},
        Object{name: "Longsword".to_owned(), cost: 40, damage: 7, armour: 0},
        Object{name: "Greataxe".to_owned(), cost: 74, damage: 8, armour: 0}];
    let armours = vec![
        Object{name: "Nothing".to_owned(), cost: 0, damage: 0, armour: 0},
        Object{name: "Leather Armour".to_owned(), cost: 13, damage: 0, armour: 1},
        Object{name: "Chainmail".to_owned(), cost: 31, damage: 0, armour: 2},
        Object{name: "Splintmail".to_owned(), cost: 53, damage: 0, armour: 3},
        Object{name: "Bandedmail".to_owned(), cost: 75, damage: 0, armour: 4},
        Object{name: "Platemail".to_owned(), cost: 102, damage: 0, armour: 5}
    ];

    let rings = vec![
        Object{name: "Nothing".to_owned(), cost: 0, damage: 0, armour: 0},
        Object{name: "Nothing".to_owned(), cost: 0, damage: 0, armour: 0},
        Object{name: "Damage +1".to_owned(), cost: 25, damage: 1, armour: 0},
        Object{name: "Damage +2".to_owned(), cost: 50, damage: 2, armour: 0},
        Object{name: "Damage +3".to_owned(), cost: 100, damage: 3, armour: 0},
        Object{name: "Defence +1".to_owned(), cost: 20, damage: 0, armour: 1},
        Object{name: "Defence +2".to_owned(), cost: 40, damage: 0, armour: 2},
        Object{name: "Defence +3".to_owned(), cost: 80, damage: 0, armour: 3},
    ];
    let mut heap = BinaryHeap::new();

    for weapon in weapons.iter(){
        for armour in armours.iter() {
            for rings in rings.iter().combinations(2){
                heap.push(Equipment{weapon: weapon.clone(), armour: armour.clone(), left_ring: rings[0].clone(), right_ring:rings[1].clone()})
            }
        }
    }
    let boss_damage = 8;
    let boss_armour = 2;
    let boss_health = 100;
    let my_health = 100;
    let mut most_gold_to_lose = 0;
    let mut least_gold_to_win = i32::MAX;
    while !heap.is_empty() {
        let equipment = heap.pop().unwrap();
        let my_turns =  boss_health / (equipment.hit_value() - boss_armour).max(1) ;
        let boss_turns =  my_health / (boss_damage - equipment.armour_value()).max(1);
        if my_turns <= boss_turns {
            least_gold_to_win = least_gold_to_win.min(equipment.total_cost());
        } else {
            most_gold_to_lose = most_gold_to_lose.max(equipment.total_cost());
        }
    }

    println!("Part 1: {}\nPart 2: {}", least_gold_to_win, most_gold_to_lose);
}
