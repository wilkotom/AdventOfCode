use std::{fs::read_to_string, collections::HashMap};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Reaction {
    quantity_produced: i64,
    reagents: HashMap<String, i64>
}


fn main() {
    let data = read_to_string("./day14/input.txt").unwrap();
    let reactions = create_mappings(&data);
    let part1_ore  = make_fuel(&reactions, "FUEL".to_owned(), 1);
    println!("Part 1: {:?}", part1_ore);
    let max_ore_available = 1_000_000_000_000;
    let mut ore_used = 1;
    let mut fuel_made = 1;
    while ore_used < max_ore_available {
        fuel_made *= 2;
        ore_used = make_fuel(&reactions, "FUEL".to_owned(), fuel_made);

    }
    let mut step = fuel_made / 2;
    let mut max_fuel_made = 0;
    while step > 0 {
        ore_used = make_fuel(&reactions, "FUEL".to_owned(), fuel_made);
        if ore_used > max_ore_available {
            fuel_made -= step;
        } else {
            max_fuel_made = max_fuel_made.max(fuel_made);
            fuel_made += step;

        }
        step /= 2;

    }
    println!("Part 2: {}", max_fuel_made);

}


fn make_fuel(reactions: &HashMap<String, Reaction>, desired_compound: String, quantity_needed: i64)  -> i64 {
    let mut satisfied = HashMap::new();
    let mut ore_required = 0;
    let mut wanted = HashMap::new();
    wanted.insert(desired_compound, quantity_needed);
    let mut need_additional_reagents = true;
    while need_additional_reagents {
        need_additional_reagents = false;
        let mut next_wanted = wanted.clone();
        for compound in wanted.keys() {
            let quantity_needed = wanted[compound] - *satisfied.get(compound).unwrap_or(&0);
            if quantity_needed > 0 {
                // integer ceiling division
                let num_reactions = (quantity_needed + reactions[compound].quantity_produced -1 ) / reactions[compound].quantity_produced;
                need_additional_reagents = true;
                let reagents = reactions[compound].reagents.clone();
                satisfied.insert(compound.to_owned(), *satisfied.get(compound).unwrap_or(&0) + reactions[compound].quantity_produced * num_reactions);
                for (reagent,quantity) in reagents {
                    if reagent == "ORE" {
                        ore_required += quantity * num_reactions
                    } else {
                        next_wanted.insert(reagent.to_owned(), next_wanted.get(&reagent).unwrap_or(&0) + quantity *  num_reactions);
                    }
                }
                
            }
        }
        wanted = next_wanted;
    }

    ore_required
}


fn create_mappings(mapping_data: &str) -> HashMap<String, Reaction>{

    let mut reactions= HashMap::new();

    for line in mapping_data.split('\n') {
        let mut sections = line.split(" => ");
        let requirements = sections.next().unwrap().split(", ");
        let mut results = sections.next().unwrap().split_ascii_whitespace();
        let quantity = results.next().unwrap().parse::<i64>().unwrap();
        let name = results.next().unwrap();
        let mut reagents = HashMap::new();
        for reagent in requirements {
            let mut split = reagent.split_ascii_whitespace();
            let quantity = split.next().unwrap().parse::<i64>().unwrap();
            let name = split.next().unwrap();
            reagents.insert(name.to_owned(), quantity);
        }
        reactions.insert(name.to_owned(), Reaction{quantity_produced: quantity, reagents});
    }
    reactions
}





#[test]
fn create_mappings_one_to_one() {
    let mapping_data = "1 ORE => 1 FUEL";
    let mappings = create_mappings(mapping_data);
    let expected_outcome = HashMap::from(
        [("FUEL".to_owned(), Reaction{quantity_produced: 1, reagents: HashMap::from([("ORE".to_owned(), 1)])})
    ]);
    assert_eq!(mappings, expected_outcome);
}

#[test]
fn create_mappings_many_to_one() {
    let mapping_data = "2 AB, 3 BC, 4 CA => 1 FUEL";
    let mappings = create_mappings(mapping_data);
    let expected_outcome = HashMap::from(
        [("FUEL".to_owned(), Reaction{quantity_produced: 1, reagents: HashMap::from([
            ("AB".to_owned(), 2),
            ("BC".to_owned(), 3),
            ("CA".to_owned(), 4),
            ])})
    ]);
    assert_eq!(mappings, expected_outcome);
}

#[test]
fn create_multiple_mappings() {
    let mapping_data = "9 ORE => 2 A
    8 ORE => 3 B
    7 ORE => 5 C
    3 A, 4 B => 1 AB
    5 B, 7 C => 1 BC
    4 C, 1 A => 1 CA
    2 AB, 3 BC, 4 CA => 1 FUEL";
    let mappings = create_mappings(mapping_data);
    let expected_outcome = HashMap::from(
        [("FUEL".to_owned(), Reaction{quantity_produced: 1, reagents: HashMap::from([
            ("AB".to_owned(), 2),
            ("BC".to_owned(), 3),
            ("CA".to_owned(), 4),
            ])}),
        ("CA".to_owned(), Reaction{quantity_produced: 1, reagents: HashMap::from([
            ("A".to_owned(), 1),
            ("C".to_owned(), 4)
        ])}),
        ("BC".to_owned(), Reaction{quantity_produced: 1, reagents: HashMap::from([
            ("B".to_owned(), 5),
            ("C".to_owned(), 7)
        ])}),
        ("AB".to_owned(), Reaction{quantity_produced: 1, reagents: HashMap::from([
            ("A".to_owned(), 3),
            ("B".to_owned(), 4)
        ])}),
        ("C".to_owned(), Reaction{quantity_produced: 5, reagents: HashMap::from([
            ("ORE".to_owned(), 7)
        ])}),
        ("B".to_owned(), Reaction{quantity_produced: 3, reagents: HashMap::from([
            ("ORE".to_owned(), 8)
        ])}),
        ("A".to_owned(), Reaction{quantity_produced: 2, reagents: HashMap::from([
            ("ORE".to_owned(), 9)
        ])}),
    ]);
    assert_eq!(mappings, expected_outcome);
}

#[test]
fn needs_165_ore() {
    let mapping_data = "9 ORE => 2 A
    8 ORE => 3 B
    7 ORE => 5 C
    3 A, 4 B => 1 AB
    5 B, 7 C => 1 BC
    4 C, 1 A => 1 CA
    2 AB, 3 BC, 4 CA => 1 FUEL";
    let ore_needed  = make_fuel(&create_mappings(mapping_data), "FUEL".to_owned(), 1);
    assert_eq!(165, ore_needed);
}

#[test]
fn needs_180697_ore() {
    let mapping_data = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
    17 NVRVD, 3 JNWZP => 8 VPVL
    53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
    22 VJHF, 37 MNCFX => 5 FWMGM
    139 ORE => 4 NVRVD
    144 ORE => 7 JNWZP
    5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
    5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
    145 ORE => 6 MNCFX
    1 NVRVD => 8 CXFTF
    1 VJHF, 6 MNCFX => 4 RFSQX
    176 ORE => 6 VJHF";
    let ore_needed  = make_fuel(&create_mappings(mapping_data), "FUEL".to_owned(), 1);
    assert_eq!(180697, ore_needed);
}


#[test]
fn needs_13312_ore() {
    let mapping_data = "157 ORE => 5 NZVS
    165 ORE => 6 DCFZ
    44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
    12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
    179 ORE => 7 PSHF
    177 ORE => 5 HKGWZ
    7 DCFZ, 7 PSHF => 2 XJWVT
    165 ORE => 2 GPVTF
    3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
    let ore_needed  = make_fuel(&create_mappings(mapping_data), "FUEL".to_owned(), 1);
    assert_eq!(13312, ore_needed);
}

#[test]
fn needs_2210736_ore() {
    let mapping_data = "171 ORE => 8 CNZTR
    7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
    114 ORE => 4 BHXH
    14 VRPVC => 6 BMBT
    6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
    6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
    15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
    13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
    5 BMBT => 4 WPTQ
    189 ORE => 9 KTJDG
    1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
    12 VRPVC, 27 CNZTR => 2 XDBXC
    15 KTJDG, 12 BHXH => 5 XCVML
    3 BHXH, 2 VRPVC => 7 MZWV
    121 ORE => 7 VRPVC
    7 XCVML => 6 RJRHP
    5 BHXH, 4 VRPVC => 5 LTCX";
    let ore_needed  = make_fuel(&create_mappings(mapping_data), "FUEL".to_owned(), 1);
    assert_eq!(2210736, ore_needed);
}
