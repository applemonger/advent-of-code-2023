use std::collections::HashMap;

use aocd::*;
use regex::Regex;

pub struct Broadcaster<'a> {
    id: &'a str,
    targets: Vec<&'a str> 
}

impl<'a> Module<'a> for Broadcaster<'a> {
    fn receive(&mut self, pulse: Pulse) -> Vec<Pulse<'a>> {
        self.targets
            .iter()
            .map(|t| {
                Pulse {
                    sender: self.id,
                    receiver: t,
                    low: pulse.low
                }
            })
            .collect()
    }

    fn id(&'a self) -> &'a str {
        self.id
    }
}

pub struct FlipFlop<'a> {
    id: &'a str,
    targets: Vec<&'a str>,
    on: bool
}

impl<'a> Module<'a> for FlipFlop<'a> {
    fn receive(&mut self, pulse: Pulse) -> Vec<Pulse<'a>> {
        if pulse.low {
            let low = self.on;
            self.on = !self.on;
            self.targets
                .iter()
                .map(|t| {
                    Pulse {
                        sender: self.id,
                        receiver: t,
                        low
                    }
                })
                .collect()
        } else {
            Vec::new()
        }
    }

    fn id(&'a self) -> &'a str {
        self.id
    }
}

pub struct Conjunction<'a> {
    id: &'a str,
    inputs: HashMap<&'a str, bool>,
    targets: Vec<&'a str>
}

impl<'a> Module<'a> for Conjunction<'a> {
    fn receive(&mut self, pulse: Pulse<'a>) -> Vec<Pulse<'a>> {
        self.inputs.insert(pulse.sender, pulse.low);
        let all_high = self.inputs.values().all(|v| !*v);
        self.targets
            .iter()
            .map(|t| {
                Pulse {
                    sender: self.id,
                    receiver: t,
                    low: all_high
                }
            })
            .collect()
    }

    fn id(&'a self) -> &'a str {
        self.id
    }
}

pub struct Untyped<'a> {
    id: &'a str
}

impl<'a> Module<'a> for Untyped<'a> {
    fn receive(&mut self, _pulse: Pulse) -> Vec<Pulse<'a>> {
        Vec::new()
    }

    fn id(&'a self) -> &'a str {
        self.id
    }
}

#[derive(Clone, Debug)]
pub struct Pulse<'a> {
    sender: &'a str,
    receiver: &'a str,
    low: bool
}

trait Module<'a> {
    fn receive(&mut self, pulse: Pulse<'a>) -> Vec<Pulse<'a>>;
    fn id(&'a self) -> &'a str;
}

fn get_sender_from_str(s: &str) -> &str {
    let sender_regex = Regex::new(r"(\w+) ->").unwrap();
    sender_regex.captures(s).unwrap().get(1).unwrap().as_str()
}

fn get_receivers_from_str(s: &str) -> Vec<&str> {
    let receivers_regex = Regex::new(r"-> (.*)$").unwrap();
    let receivers = receivers_regex.captures(s).unwrap().get(1).unwrap().as_str();
    let receivers: Vec<&str> = receivers.trim().split(", ").collect();
    receivers
}

pub enum ModuleType {
    Broadcaster,
    FlipFlop,
    Conjunction,
    Untyped
}

fn get_module_type_from_str(s: &str) -> ModuleType {
    let sender_with_prefix_regex = Regex::new(r"(.+) ->").unwrap();
    let sender_with_prefix = sender_with_prefix_regex.captures(s).unwrap().get(1).unwrap().as_str();
    if sender_with_prefix == "broadcaster" {
        ModuleType::Broadcaster
    } else if sender_with_prefix.contains('%') {
        ModuleType::FlipFlop
    } else if sender_with_prefix.contains('&') {
        ModuleType::Conjunction
    } else {
        unreachable!("Sender must be one of the three types above.")
    }
}

fn get_all_module_names_from_str(s: &str) -> Vec<&str> {
    let sender = get_sender_from_str(s);
    let receivers_regex = Regex::new(r"-> (.*)$").unwrap();
    let receivers = receivers_regex.captures(s).unwrap().get(1).unwrap().as_str();
    let mut receivers: Vec<&str> = receivers.trim().split(", ").collect();
    let mut all_module_names = Vec::new();
    all_module_names.push(sender);
    all_module_names.append(&mut receivers);
    all_module_names
}

pub struct ModuleData<'a> {
    module: ModuleType,
    sender: &'a str,
    receivers: Vec<&'a str>,
    inputs: Vec<&'a str>
}

impl<'a> ModuleData<'a> {
    fn new(sender: &str) -> ModuleData {
        ModuleData {
            module: ModuleType::Untyped,
            sender,
            receivers: Vec::<&str>::new(),
            inputs: Vec::<&str>::new()
        }
    }

    fn to_module(self) -> Box<dyn Module<'a> + 'a> {
        match self.module {
            ModuleType::Broadcaster => {
                Box::new(Broadcaster {
                    id: self.sender,
                    targets: self.receivers
                })
            },
            ModuleType::Conjunction => {
                Box::new(Conjunction {
                    id: self.sender,
                    targets: self.receivers,
                    inputs: self.inputs.into_iter().map(|s| (s, true)).collect()
                })
            },
            ModuleType::FlipFlop => {
                Box::new(FlipFlop {
                    id: self.sender,
                    targets: self.receivers,
                    on: false
                })
            },
            ModuleType::Untyped => {
                Box::new(Untyped {
                    id: self.sender
                })
            }
        }
    }
}

#[aocd(2023, 19, "src/day20/input.txt")]
pub fn solution1() {
    let input_data = input!();

    // Get names of all referenced modules in senders or receivers
    let module_names: Vec<&str> = input_data
        .split('\n')
        .flat_map(get_all_module_names_from_str)
        .collect();
    
    // Data structure to store all pertinent module data for each referenced module
    let mut module_data: HashMap<&str, ModuleData> = module_names
        .iter()
        .map(|s| {
            (*s, ModuleData::new(s))
        })
        .collect();

    // Loop back through the input data and fill the data structure
    for data in input_data.split('\n') {
        let sender = get_sender_from_str(data);
        let receivers = get_receivers_from_str(data);
        let module_type = get_module_type_from_str(data);
        let module = module_data.get_mut(&sender).unwrap();
        module.receivers = receivers;
        module.module = module_type;
    }        
    
    // Loop back through the input data and fill in input modules
    for data in input_data.split('\n') {
        let sender = get_sender_from_str(data);
        let receivers = get_receivers_from_str(data);
        // For each receiver listed, find that module and add in the sender as an input
        for receiver in receivers.iter() {
            let module = module_data.get_mut(receiver).unwrap();
            module.inputs.push(sender);
        }
    }

    // Create modules from the module data
    let mut modules: HashMap<&str, Box<dyn Module>> = module_data.into_iter()
        .map(|(module_name, module_data)| {
            (module_name, module_data.to_module())
        })
        .collect();

    let mut low_count: u64 = 0;
    let mut high_count: u64 = 0;
    for _ in 0..1000 {
        let button_pulse = Pulse {
            sender: "button",
            receiver: "broadcaster",
            low: true
        };
        let mut pulse_queue = vec![button_pulse];
        while !pulse_queue.is_empty() {
            let current_pulse = pulse_queue.remove(0);
            if current_pulse.low {
                low_count += 1;
            } else {
                high_count += 1;
            }
            let module = modules.get_mut(current_pulse.receiver).unwrap();
            let mut pulses = module.receive(current_pulse);
            pulse_queue.append(&mut pulses);
        }
    }

    submit!(1, low_count * high_count);
}

#[aocd(2023, 19)]
pub fn solution2() {}
