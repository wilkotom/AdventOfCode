use std::fs::read_to_string;

#[derive(Debug,Copy,Clone)]
struct Sue{
    id: i32,
    children: Option<i32>,
    cats: Option<i32>,
    samoyeds: Option<i32>,
    pomeranians: Option<i32>,
    akitas: Option<i32>,
    vizslas: Option<i32>,
    goldfish: Option<i32>,
    trees: Option<i32>,
    cars: Option<i32>,
    perfumes: Option<i32>
}

fn main() {
    let data = read_to_string("./input.txt").unwrap();
    let sought_sue = Sue{id: 0,
        children: Some(3),cats: Some(7), samoyeds: Some(2),
        pomeranians: Some(3),
        akitas: Some(0),
        vizslas: Some(0),
        goldfish: Some(5),
        trees: Some(3),
        cars: Some(2),
        perfumes: Some(1),
    };
    for line in data.split('\n') {
        let mut fields = line.split_ascii_whitespace();
        fields.next();
        let id_str = fields.next().unwrap();
        let id = id_str[..id_str.len()-1].parse::<i32>().unwrap();
        let mut children: Option<i32> = None;
        let mut cats: Option<i32> = None;
        let mut samoyeds: Option<i32> = None;
        let mut pomeranians: Option<i32> = None;
        let mut akitas: Option<i32> = None;
        let mut vizslas: Option<i32> = None;
        let mut goldfish: Option<i32> = None;
        let mut trees: Option<i32> = None;
        let mut cars: Option<i32> = None;
        let mut perfumes: Option<i32> = None;
        while let Some(thing) = fields.next() {
            let number = fields.next().unwrap(); 
            let quantity = Some((if let Some(num) = number.strip_suffix(',') {num} else {number}).parse::<i32>().unwrap());
            match thing.strip_suffix(':').unwrap() {
                "children"    => { children = quantity; },
                "cats"        => { cats = quantity; },
                "samoyeds"    => { samoyeds = quantity; },
                "pomeranians" => { pomeranians = quantity; },
                "akitas"      => { akitas = quantity;},
                "vizslas"     => { vizslas = quantity; },
                "goldfish"    => { goldfish = quantity; },
                "trees"       => { trees = quantity; },
                "cars"        => { cars = quantity; },
                "perfumes"    => { perfumes = quantity;},
                t => { println!("Thing? {}", t);
                            unimplemented!()
                }
            }
    
        }
        let this_sue = Sue{id, children, cats, samoyeds, pomeranians, akitas, vizslas, goldfish, trees, cars, perfumes};
        if (this_sue.children.is_none() || this_sue.children == sought_sue.children) && 
            (this_sue.cats.is_none() || this_sue.cats == sought_sue.cats) && 
            (this_sue.samoyeds.is_none() || this_sue.samoyeds == sought_sue.samoyeds) && 
            (this_sue.pomeranians.is_none() || this_sue.pomeranians == sought_sue.pomeranians) && 
            (this_sue.akitas.is_none() || this_sue.akitas == sought_sue.akitas) && 
            (this_sue.vizslas.is_none() || this_sue.vizslas == sought_sue.vizslas) &&
            (this_sue.goldfish.is_none() || this_sue.goldfish == sought_sue.goldfish) &&
            (this_sue.trees.is_none() || this_sue.trees == sought_sue.trees) &&
            (this_sue.cars.is_none() || this_sue.cars == sought_sue.cars) &&
            (this_sue.perfumes.is_none() || this_sue.perfumes == sought_sue.perfumes) {
                println!("Part 1: {}", this_sue.id);
        }
        if (this_sue.children.is_none() || this_sue.children == sought_sue.children) && 
            (this_sue.cats.is_none() || this_sue.cats > sought_sue.cats) && 
            (this_sue.samoyeds.is_none() || this_sue.samoyeds == sought_sue.samoyeds) && 
            (this_sue.pomeranians.is_none() || this_sue.pomeranians < sought_sue.pomeranians) && 
            (this_sue.akitas.is_none() || this_sue.akitas == sought_sue.akitas) && 
            (this_sue.vizslas.is_none() || this_sue.vizslas == sought_sue.vizslas) &&
            (this_sue.goldfish.is_none() || this_sue.goldfish < sought_sue.goldfish) &&
            (this_sue.trees.is_none() || this_sue.trees > sought_sue.trees) &&
            (this_sue.cars.is_none() || this_sue.cars == sought_sue.cars) &&
            (this_sue.perfumes.is_none() || this_sue.perfumes == sought_sue.perfumes) {
                println!("Part 2: {}", this_sue.id);
        }
    }
}
