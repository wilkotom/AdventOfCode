use std::{error::Error, collections::{HashMap, VecDeque}};
use aochelpers::{get_daily_input, lcm, Label};

#[derive(Debug,Clone, Copy,PartialEq)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcast
}

#[derive(Debug,Clone,Copy, PartialEq)]
enum ModuleState {
    On,
    Off,
}

#[derive(Debug,Clone, Copy,PartialEq, Eq)]
enum Pulse {
    Low, 
    High, 
    None
}

#[derive(Debug,Clone,PartialEq)]
struct CommunicationModule {
    kind: ModuleType,
    state: ModuleState,
    next_modules: Vec<Label>,
    memory: HashMap<Label,Pulse>

}

#[derive(Debug,Clone,PartialEq, Eq)]
struct PulsePacket{
    sender: Label,
    receiver: Label,
    pulse: Pulse
}

#[derive(Debug,Clone,PartialEq, Eq)]
struct TerminationCondition{
    sender: Label,
    pulse: Pulse
}

impl CommunicationModule {
    fn receive(&mut self, pulse: &Pulse, sender: Label) -> Pulse{
        match self.kind {
            ModuleType::FlipFlop => {
                if *pulse == Pulse::Low  && self.state == ModuleState::Off {
                    self.state = ModuleState::On;
                    Pulse::High
                } else if *pulse == Pulse::Low && self.state == ModuleState::On {
                    self.state = ModuleState::Off;
                    Pulse::Low
                } else {
                    Pulse::None
                }
            },
            ModuleType::Conjunction => {
                self.memory.insert(sender, *pulse);
                if self.memory.values().all(|x| *x == Pulse::High) {
                    Pulse::Low
                } else {
                    Pulse::High
                }
            }
            ModuleType::Broadcast => *pulse,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>>{
    let data = get_daily_input(20,2023)?;
    part1(&data);
    part2(&data);
    Ok(())
}

fn part1(data: &str) {
    let mut network: HashMap<Label, CommunicationModule> = parse_data(data);
    let mut low: usize = 0;
    let mut high = 0;
    for _ in 0..1000{
        let(l,h, _) = press_button(&mut network, None);
        low += l;
        high += h;
    }
    println!("Part 1: {}", low * high);
}

fn part2(data: &str) {
    // Find what leads to the termination condition
    let network: HashMap<Label, CommunicationModule> = parse_data(data);
    // Assumption: Only one module can feed to rx, and it is a Conjunction Module
    for (module, details) in network.iter() {
        for next_module_name in details.next_modules.iter(){
            if *next_module_name == "rx".parse::<Label>().unwrap() {
                let mut cycles = Vec::new();
                let previous_stage: &CommunicationModule = network.get(module).unwrap();
                for feeder_stage in previous_stage.memory.keys() {
                    // If I really cared about performance I'd reset an existing instance of the network
                    // instead of parsing the input again. I don't care about performance.
                    let mut network: HashMap<Label, CommunicationModule> = parse_data(data); 
                    let cycle_time = part2_subcondition(&mut network,
                         &TerminationCondition{pulse: Pulse::High, sender: *feeder_stage});
                    cycles.push(cycle_time);
                }
                // Time for rx to be triggered is LCM of each of the cycles for rx's input's inputs 
                println!("Part 2: {}", cycles[1..].iter().fold(cycles[0], |c: usize,v| lcm(c, *v)));
                return;
            }
        }
    }
}

fn part2_subcondition(network: &mut HashMap<Label,CommunicationModule>, termination_condition: &TerminationCondition) -> usize {
    let mut counter = 0;
    loop {
        let (_,_, satisfied ) = press_button(network, Some(termination_condition));
        counter+=1;
        if satisfied {
            return counter;
        }
    }
}

fn press_button(network: &mut HashMap<Label,CommunicationModule>, termination_condition: Option<&TerminationCondition>) -> (usize, usize, bool){
    let mut high_pulses = 0;
    let mut low_pulses = 0;
    let mut unprocessed = VecDeque::new();
    unprocessed.push_back(PulsePacket{sender: "button".parse::<Label>().unwrap(), receiver: "broadcaster".parse::<Label>().unwrap(), pulse: Pulse::Low});
    while let Some(message) = unprocessed.pop_front() {
        match message.pulse {
            Pulse::Low => {low_pulses += 1},
            Pulse::High => {high_pulses +=1},
            Pulse::None => {continue}
        };
        if let Some(tc) = termination_condition {
            if message.sender == tc.sender && message.pulse == tc.pulse {
                return (0,0,true)
            }
        }
        if !network.contains_key(&message.receiver) { // Messages going to "rx" or "output"
            continue;
        }
        let receiver = network.get_mut(&message.receiver).unwrap();
        let result = receiver.receive(&message.pulse, message.sender);

        for module in receiver.next_modules.iter() {
            let packet = PulsePacket{
                sender: message.receiver,
                receiver: module.to_owned(),
                pulse: result
            };
            unprocessed.push_back(packet);
        }
    }
    (low_pulses, high_pulses, false)
}

fn parse_data(data:&str) -> HashMap<Label, CommunicationModule> {
    let mut modules = HashMap::new();
    for line in data.lines() {
        let mut sections = line.split(" -> ");
        let name = sections.next().unwrap();
        if let Some(label) = name.strip_prefix('%') {
            modules.insert(label.parse::<Label>().unwrap(), CommunicationModule {
                kind: ModuleType::FlipFlop,
                state: ModuleState::Off,
                next_modules: sections.next().unwrap().split(", ").map(|x| x.parse::<Label>().unwrap()).collect::<Vec<_>>(),
                memory: HashMap::new()
            });
        } else if let Some(label) = name.strip_prefix('&') {
            modules.insert(label.parse::<Label>().unwrap(), CommunicationModule {
                kind: ModuleType::Conjunction,
                state: ModuleState::Off,
                next_modules: sections.next().unwrap().split(", ").map(|x| x.parse::<Label>().unwrap()).collect::<Vec<_>>(),
                memory: HashMap::new()
            });
        } else {
            modules.insert(name.parse::<Label>().unwrap(), CommunicationModule {
                kind: ModuleType::Broadcast,
                state: ModuleState::Off,
                next_modules: sections.next().unwrap().split(", ").map(|x| x.parse::<Label>().unwrap()).collect::<Vec<_>>(),
                memory: HashMap::new()
            });
        }
    }
    for (module, details) in modules.clone().iter() {
        for next_module_name in details.next_modules.iter(){
            if !modules.contains_key(next_module_name) {
                continue;
            }
            let next_module  = modules.get_mut(next_module_name).unwrap();
            if next_module.kind == ModuleType::Conjunction {
                next_module.memory.insert(*module, Pulse::Low);
            }
        }
    }
    modules
}

#[cfg(test)]
mod tests {

    use super::*; 
    const EXAMPLE1:&str = 
"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    const EXAMPLE2: &str = 
"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";


    #[test]
    fn test_example1() {
        let mut network: HashMap<Label, CommunicationModule> = parse_data(EXAMPLE1);
        assert_eq!(press_button(&mut network, None), (8,4, false));
    }

    #[test]
    fn test_example2_by_stage() {
        let mut network: HashMap<Label, CommunicationModule> = parse_data(EXAMPLE2);
        let label_a = "a".parse::<Label>().unwrap();
        let label_b = "b".parse::<Label>().unwrap();
        assert_eq!(network[&label_a].state, ModuleState::Off);
        assert_eq!(network[&label_b].state, ModuleState::Off);
        press_button(&mut network, None);
        assert_eq!(network[&label_a].state, ModuleState::On);
        assert_eq!(network[&label_b].state, ModuleState::On);
        press_button(&mut network, None);
        assert_eq!(network[&label_a].state, ModuleState::Off);
        assert_eq!(network[&label_b].state, ModuleState::On);
        press_button(&mut network, None);
        assert_eq!(network[&label_a].state, ModuleState::On);
        assert_eq!(network[&label_b].state, ModuleState::Off);
        press_button(&mut network, None);
        assert_eq!(network[&label_a].state, ModuleState::Off);
        assert_eq!(network[&label_b].state, ModuleState::Off);

    }
    #[test]
    fn test_example2() {
        let mut network: HashMap<Label, CommunicationModule> = parse_data(EXAMPLE2);
        let mut low: usize = 0;
        let mut high = 0;
        for _ in 0..1000{
            let(l,h, _) = press_button(&mut network, None);
            low += l;
            high += h;
        }
        assert_eq!((low,high),(4250,2750));
    }
}