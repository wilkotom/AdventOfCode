use std::{fs::read_to_string, collections::HashMap, cmp::Reverse};

#[derive(Debug, Clone)]
struct AttackGroup {
    unit_count: i32,
    unit_hit_points: i32,
    weaknesses: Vec<String>,
    immunities: Vec<String>,
    initiative: i32,
    attack_damage: i32,
    attack_type: String,
    // is_immune_system: bool,
    id: usize,
    boost: i32
}

impl AttackGroup{
    fn effective_power(&self) -> i32{
        self.unit_count * (self.attack_damage + self.boost)
    }

    fn damage_done(&self, other: &Self) -> i32 {
        self.effective_power() * if other.weaknesses.contains(&self.attack_type) {
            2
        } else if other.immunities.contains(&self.attack_type) {
            0
        } else {
            1
        }
    }

    fn targeting_order(&self) -> (i32,i32) {
        (self.effective_power(), self.initiative)
    }
}


fn main() {
    let mut battlefield = parse_input("./input.txt");
    let mut battle_results = combat(battlefield.clone());
    println!("Part 1: {:?}", battle_results.0.max(battle_results.1));
    let mut boost = 1;
    let mut best_score = i32::MAX;
    while battle_results.1 != 0 {
        boost *= 2;
        for unit in battlefield.0.iter_mut() {
            unit.boost = boost;
        }
        battle_results = combat(battlefield.clone());
    }
    let mut window_size = boost / 2;
    boost -= window_size;
    while window_size > 0 {
        for unit in battlefield.0.iter_mut() {
            unit.boost = boost;
        }
        battle_results = combat(battlefield.clone());
        if battle_results.1 == 0 {
            best_score = best_score.min(battle_results.0);
        }
        if battle_results.1 != 0 {
            boost += window_size / 2;
        } else {
            boost -= window_size /2;
        }
        window_size /=2;
    }
    println!("Part 2 answer: {}", best_score);
}

fn combat(battlefield: (Vec<AttackGroup>, Vec<AttackGroup>)) -> (i32, i32){
    let mut immune_system = battlefield.0;
    let mut infection = battlefield.1;
    let mut last_result = (-1,-1);
    let immune_system_result = immune_system.iter().map(|g| g.unit_count).sum::<i32>();
    let infection_result = infection.iter().map(|g| g.unit_count).sum::<i32>();
    let mut new_result = (immune_system_result, infection_result);

    while immune_system.iter().filter(|g| g.unit_count > 0).count() > 0 && infection.iter().filter(|g| g.unit_count > 0).count() > 0 && last_result != new_result {
        last_result = new_result;
        immune_system.sort_by_key(|b| Reverse(b.targeting_order()));
        infection.sort_by_key(|b| Reverse(b.targeting_order()));
        let mut immune_system_targets = HashMap::new();
        let mut infection_targets = HashMap::new();

        for unit in &infection{
            if unit.unit_count <= 0 {
                continue;
            }
            let mut max_damage = 0;
            for target in &immune_system {
                if ! infection_targets.values().any(|x| x == &target.id)  && target.unit_count >0{
                    let damage = unit.damage_done(target);
                    if damage > max_damage && ! infection_targets.values().any(|x| x == &target.id) {
                        max_damage = damage;
                        infection_targets.insert(unit.id, target.id);
                    }
                }
            }
        }

        for unit in &immune_system{
            if unit.unit_count <= 0 {
                continue;
            }
            let mut max_damage = 0;
            for target in &infection {
                if ! immune_system_targets.values().any(|x| x == &target.id)  && target.unit_count >0 {

                    let damage = unit.damage_done(target);
                    if damage > max_damage && ! immune_system_targets.values().any(|x| x == &target.id) {
                        max_damage = damage;
                        immune_system_targets.insert(unit.id, target.id);
                    }
                }
            }
        }
        let mut initiatives = immune_system.iter().map(|g| g.initiative).collect::<Vec<_>>();
        initiatives.append(&mut infection.iter().map(|g| g.initiative).collect::<Vec<_>>());
        initiatives.sort_by(|a,b| b.cmp(a));
        for initiative in initiatives {
            for immune_unit in &immune_system {
                if immune_unit.initiative == initiative  && immune_unit.unit_count > 0{
                   for target in infection.iter_mut() {
                       if immune_system_targets.contains_key(&immune_unit.id) && target.id == immune_system_targets[&immune_unit.id] {
                           let damage_done = immune_unit.damage_done(target);
                           let units_killed = target.unit_count.min(damage_done / target.unit_hit_points);
                           target.unit_count -= units_killed;
                           
                       }
                   }

                }
            }

            for infection_unit in &infection {
                if infection_unit.initiative == initiative && infection_unit.unit_count > 0{
                   for target in immune_system.iter_mut() {
                       if infection_targets.contains_key(&infection_unit.id) && target.id == infection_targets[&infection_unit.id] {
                           let damage_done = infection_unit.damage_done(target);
                           let units_killed = target.unit_count.min(damage_done / target.unit_hit_points);
                           target.unit_count -= units_killed;

                       }
                   }

                }
            }
            let immune_system_result = immune_system.iter().map(|g| g.unit_count).sum::<i32>();
            let infection_result = infection.iter().map(|g| g.unit_count).sum::<i32>();
            new_result = (immune_system_result, infection_result);
        }
    }
    new_result
}


fn parse_input(filename: &str) -> (Vec<AttackGroup>, Vec<AttackGroup>) {
    let input = read_to_string(filename).unwrap();
    let mut armies = input.split("\n\n");
    let immune_system =  return_attack_groups(armies.next().unwrap());
    let infection =  return_attack_groups(armies.next().unwrap());
    (immune_system, infection)
}

fn return_attack_groups(raw_input: &str) -> Vec<AttackGroup> {

    let mut lines = raw_input.split('\n');

    let mut attack_groups = Vec::new();

    lines.next();
    for (id, line) in lines .enumerate(){
        let mut weaknesses: Vec<String> = Vec::new();
        let mut immunities: Vec<String> = Vec::new();    
        let mut words = line.split_ascii_whitespace();
        let unit_count: i32 = words.next().unwrap().parse().unwrap();
        words.next();
        words.next();
        words.next();
        let unit_hit_points: i32 = words.next().unwrap().parse().unwrap();
        
        words.next();
        words.next();
        if line.contains('(') {
            // this bit is truly horrible. In fact this whole function is ridiculous.
           let mut section = Vec::new();
           for w in &mut words {
               section.push(w);
                if w.ends_with(')') {
                    break;
                }
           }
           let clarification = section.join(" ").replace(",", "");
           let sections = clarification[1..clarification.len()-1].split("; ");
           
           for section in sections {
                let details = section.split_ascii_whitespace().collect::<Vec<_>>();
                if details[0] == "immune" {
                    immunities = details[2..].iter().map(|x| x.to_string()).collect::<Vec<_>>();
                } else {
                    weaknesses = details[2..].iter().map(|x| x.to_string()).collect::<Vec<_>>();
                }
           }
        }
        words.next();
        words.next();
        words.next();
        words.next();
        words.next();
        let attack_power: i32 = words.next().unwrap().parse().unwrap();
        let attack_type = words.next().unwrap().to_owned();
        let initiative:i32 = words.last().unwrap().parse().unwrap();

        let attack_group = AttackGroup{ 
            unit_count, 
            unit_hit_points, 
            weaknesses: weaknesses.iter().map(|x| x.to_string()).collect::<Vec<_>>(), 
            immunities: immunities.iter().map(|x| x.to_string()).collect::<Vec<_>>(), 
            initiative, attack_damage: attack_power, 
            attack_type, 
            // is_immune_system, 
            id: id+1,
            boost: 0
            };

    
        attack_groups.push(attack_group);
    }
    attack_groups
}
// 16489 too low!

//18243 too high