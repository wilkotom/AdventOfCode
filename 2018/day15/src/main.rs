use std::{collections::{HashSet,HashMap,BinaryHeap}, cmp::Ordering};

/*
There are many, many optimisations I could make in this code. But this is probabvly my 
most-hated puzzle of all time. It gives a correct answer for my input, I'm not going to
spend time making it faster even though there are obvious quick wins all over the place.
*/


#[derive(Debug,PartialEq, Eq,Clone, Copy)]
enum ActorType {
    Elf,
    Goblin
}

#[derive(Debug,Clone,Copy,Hash, PartialEq, Eq)]
struct Coordinate {
    x: usize,
    y: usize
}

impl Coordinate {
    fn neighbours(&self) -> [Coordinate; 4] {
        [
            Coordinate{x: self.x, y: self.y-1 },
            Coordinate{x: self.x-1, y: self.y },
            Coordinate{x: self.x+1, y: self.y },
            Coordinate{x: self.x, y: self.y+1 },
        ]

    }
}

impl Ord for Coordinate {
    fn cmp(&self, other: &Self) -> Ordering{
        if other.y == self.y {
            self.x.cmp(&other.x)
        } else {
            self.y.cmp(&other.y)
        }
    }
}

impl PartialOrd for Coordinate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>{
        Some(self.cmp(&other))
    }
}

#[derive(Debug,PartialEq, Eq,Copy,Clone)]
struct Actor {
    health: i32,
    attack: i32,
    side: ActorType
}

#[derive(Debug,PartialEq, Eq, Clone, Copy)]
enum Square {
    Empty,
    Wall,
    Actor(Actor)
}

#[derive(PartialEq, Eq)]
struct SquareDistance {
    square: Coordinate,
    distance: usize
}

impl Ord for SquareDistance {
    fn cmp(&self, other: &Self) -> Ordering{
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for SquareDistance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>{
        Some(self.cmp(&other))
    }


}

fn main() {
    let map = read_map("./input.txt");
    println!("Part 1: {}", play_game(map.clone(), false).0);
    let mut elf_attack = 3;
    let mut aborted = true;
    let mut result = 0;
    while aborted {
        let mut new_map = map.clone();
        for square in new_map.values_mut() {
            if let Square::Actor(x) = square {
                if x.side == ActorType::Elf {
                    x.attack = elf_attack
                }
            }
        }
        (result, aborted) = play_game(new_map, true);
        elf_attack += 1;

    }
    println!("Part 2: {}", result)

}


fn read_map(filename: &str) -> HashMap<Coordinate, Square>{
    let mut arena = HashMap::new();
    let raw_data = std::fs::read_to_string(filename).unwrap();
    for (y, line) in raw_data.split('\n').enumerate() {
        for (x,c ) in line.chars().enumerate() {
            let square = match c {
                '#' => Square::Wall,
                '.' => Square::Empty,
                'E' => Square::Actor(Actor{health: 200, attack: 3, side: ActorType::Elf}),
                'G' => Square::Actor(Actor{health: 200, attack: 3, side: ActorType::Goblin}),
                _ => unimplemented!()
            };

            arena.insert(Coordinate{x,y}, square);

        }
    }
    arena
}


fn play_game (mut arena: HashMap<Coordinate,Square>, abort_on_elf_death: bool) -> (i32,bool) {

    let (top_left, bottom_right) = arena_dimensions(&arena);
    let mut rounds = 0;
    let mut aborted = false;
    'outer: loop {
        // Scan the board for all actors
        let mut actor_locations = vec![];
        for y in top_left.y..=bottom_right.y {
            for x in top_left.x..=bottom_right.x {
                if let Some(Square::Actor(_)) = arena.get(&Coordinate { x, y}) {
                    actor_locations.push(Coordinate{x,y})
                }
            }
        }
        for mut location in actor_locations {
            // if the actor is already dead, skip

            match arena.get(&location) {
                // if the actor is already dead, skip
                Some(Square::Empty) | Some (Square::Wall) => {},
                Some(Square::Actor(a)) => {
                    let a = *a;
                    let target_squares = get_targets(&a, &arena);
                    if target_squares.is_empty() {
                        break 'outer;
                    }
                    let distances = get_distances(location, &arena);
                    
                    // Find the target that is the fewest number of steps away
                    let mut distance = usize::MAX;
                    let mut chosen = Coordinate{x:0, y:0};
                    for t in target_squares {
                        if let Some(d) = distances.get(&t) {
                            if  *d < distance {
                                distance = *d;
                                chosen = t;
                            } else if *d == distance && t < chosen {
                                chosen = t;
                            }
                        }
                    }
                    // If there's a suitable (reachable) target that requires movement
                    if distance > 0 && distance < usize::MAX {
                        let mut distances_by_next = HashMap::new();
                        // work out the distance from each of the adjacent squares to the target
                        for neighbour in location.neighbours() {
                            if arena.get(&neighbour) == Some(&Square::Empty) {
                            
                                distances_by_next.insert(neighbour, get_distances(neighbour, &arena).clone());
                            }
                        }
                        // pick the one that has the shortest path to the target, using (y,x) as a tie-breaker
                        let maybe_next = &location.neighbours()
                            .iter()
                            .filter(|s| arena.get(s) == Some(&Square::Empty))
                            .min_by(|a,b| distances_by_next.get(a)
                                    .unwrap()
                                    .get(&chosen)
                                    .unwrap_or(&usize::MAX)
                                    .cmp(distances_by_next
                                        .get(b)
                                        .unwrap()
                                        .get(&chosen)
                                        .unwrap_or(&usize::MAX)))
                            .copied();
                        // move to that square
                        if let Some(next_square) = maybe_next {
                            arena.insert(*next_square, Square::Actor(a));
                            arena.insert(location, Square::Empty);
                            location = *next_square;
                        }
                    }
                    let mut selected_target = None;
                    let mut target_health = i32::MAX;
                    for neighbour in location.neighbours() {
                        
                        if let Some(Square::Actor(t)) = arena.get(&neighbour){
                            if t.side != a.side && t.health < target_health {
                                selected_target = Some(neighbour);
                                target_health = t.health;
                            }
                        }
                    }
                    if let Some(s) = selected_target {
                        if let Some(Square::Actor(t)) = arena.get_mut(&s){
                            t.health -= a.attack;
                            if t.health <= 0 {
                                if t.side == ActorType::Elf && abort_on_elf_death{
                                    arena.insert(selected_target.unwrap(), Square::Empty);
                                    aborted = true;
                                    break;
                                }
                                arena.insert(selected_target.unwrap(), Square::Empty);

                            }
                        }
                        
                    }
                }
                None => unimplemented!()
            }
            if arena.get(&location) == Some(&Square::Empty) { 
                continue;
            }
        }
        rounds +=1;
    }
    display_arena(&arena);

    let mut health_total = 0;
    for square in arena.values() {
        if let Square::Actor(a) = square {
            health_total += a.health;
        }
    }
    (health_total * rounds, aborted)
}


fn arena_dimensions(arena: &HashMap<Coordinate,Square>) -> (Coordinate, Coordinate) {
    let mut top_left = Coordinate{ x: usize::MAX, y: usize::MAX};
    let mut bottom_right = Coordinate{x: usize::MIN, y: usize::MIN};

    for coord in arena.keys() {
        top_left.x = top_left.x.min(coord.x);
        top_left.y = top_left.y.min(coord.x);
        bottom_right.x = bottom_right.x.max(coord.x);
        bottom_right.y = bottom_right.y.max(coord.y);
    }
    (top_left, bottom_right)
}

fn get_targets(attacker: &Actor, arena: &HashMap<Coordinate,Square>) -> HashSet<Coordinate>{
    let mut targets = HashSet::new();
    for (k,v) in arena.iter() {
        if let Square::Actor(defender) = v {
            if attacker.side != defender.side {
                for neighbour in k.neighbours() {
                    targets.insert(neighbour);
                }
            }
        }
    }
    targets
}

fn get_distances(location: Coordinate, arena: &HashMap<Coordinate,Square>) -> HashMap<Coordinate, usize> {
    let mut results: HashMap<Coordinate, usize> = HashMap::new();
    let mut unvisited = BinaryHeap::new();
    results.insert(location, 0);
    for neighbour in location.neighbours() {
        unvisited.push(SquareDistance { square: neighbour, distance: 1 });
    }


    while let Some(square) = unvisited.pop() {
        if results.contains_key(&square.square) {
            continue;
        }
        match arena.get(&square.square) {
            Some(Square::Wall) | Some(Square::Actor(_)) => {}
            Some(_) => {
                results.insert(square.square, square.distance);
                for neighbour in square.square.neighbours() {
                    unvisited.push(SquareDistance { square: neighbour, distance: square.distance+1 });
                }
            },
            None => unreachable!(),
        }
    }

    results
}


fn display_arena(arena: &HashMap<Coordinate,Square>) {
    let (top_left, bottom_right) = arena_dimensions(&arena);

    for y in top_left.y..=bottom_right.y {
        let mut health_details = vec![];
        for x in top_left.x ..= bottom_right.x {
            match arena.get(&Coordinate { x,  y}) {
                Some(Square::Wall) => print!("#"),
                Some(Square::Empty) => print!(" "),
                Some(Square::Actor(a)) if a.side == ActorType::Elf => {
                    print!("E");
                    health_details.push(format!("E({})", a.health))
                },
                Some(Square::Actor(a)) if a.side == ActorType::Goblin => {
                    print!("G");
                    health_details.push(format!("G({})", a.health))
                },
                _=> unimplemented!()
            };
        }
        println!("    {}", health_details.join(", "));
    }

}