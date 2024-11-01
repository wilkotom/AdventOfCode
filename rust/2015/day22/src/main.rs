use std::{collections::BinaryHeap, cmp::Ordering};


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct GameState {
    turn: i32,
    player_health: i32,
    player_mana: i32,
    boss_health: i32,
    boss_damage: i32,
    shield_timer: i32,
    poison_timer: i32,
    recharge_timer: i32,
    mana_spent: i32
}

impl Ord for GameState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.mana_spent.cmp(&self.mana_spent)
    }
}

impl PartialOrd for GameState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let starting_state = GameState{
        turn: 0,
        player_health: 50,
        player_mana: 500,
        boss_health: 51,
        boss_damage: 9,
        shield_timer: 0,
        poison_timer: 0,
        recharge_timer: 0,
        mana_spent: 0
    };
    let mut heap = BinaryHeap::new();
    heap.push(starting_state);
    while !heap.is_empty() {
        let mut state = heap.pop().unwrap();
        if state.turn % 2 == 0 {
            state.player_health -= 1;
        }
        if state.player_health <= 0 || state.player_mana < 53 {
            continue;
        }    
        
        if state.poison_timer > 0 {
            state.boss_health -=3;
            state.poison_timer -=1;
        }
        if state.boss_health <= 0 {
            println!("Boss killed");
            println!("Mana Spent: {}", state.mana_spent);
            break;
        }
        if state.recharge_timer > 0 {
            state.player_mana += 101;
            state.recharge_timer -=1;
        }
        state.shield_timer = 0.max(state.shield_timer -1);
        state.turn +=1;
        if state.turn %2 == 1 {
            // cast magic missile?
            if state.player_mana >= 53 {
                let mut next_state = state;
                next_state.boss_health -=4;
                next_state.player_mana -= 53;
                next_state.mana_spent += 53;
                heap.push(next_state);
            }

            // cast drain?
            if state.player_mana >= 73 {
                let mut next_state = state;
                next_state.boss_health -=2;
                next_state.player_health +=2;
                next_state.player_mana -=73;
                next_state.mana_spent += 73;
                heap.push(next_state);
            }

            // cast shield?
            if state.player_mana >= 113 && state.shield_timer == 0 {
                let mut next_state = state;
                next_state.shield_timer = 6;
                next_state.player_mana -=113;
                next_state.mana_spent += 113;
                heap.push(next_state);
            }

            // Cast Poison
            if state.player_mana >= 173 && state.poison_timer == 0 {
                let mut next_state = state;
                next_state.poison_timer = 6;
                next_state.player_mana -= 173;
                next_state.mana_spent += 173;
                heap.push(next_state);
            }

            // Cast Recharge
            if state.player_mana >= 229 && state.recharge_timer == 0{
                let mut next_state = state;
                next_state.recharge_timer = 5;
                next_state.player_mana -= 229;
                next_state.mana_spent += 229;
                heap.push(next_state);
            }
        
        } else {
            state.player_health -= if state.shield_timer > 0 {state.boss_damage - 7} else {state.boss_damage};
            if state.player_health > 0 {
                heap.push(state);
            }
            
        }

    }
    
}
