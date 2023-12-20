use std::{error::Error, fs::File, io::{BufReader, BufRead}};
use std::collections::HashMap;
use crate::ModuleType::*;
use crate::Pulse::*;

#[derive(Debug, PartialEq)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcast,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Pulse {
    High,
    Low,
}

impl Pulse {
    fn flip(&mut self) {
        *self = match &self {
            High => Low,
            Low => High,
        };
    }
}

fn main()  -> Result<(), Box<dyn Error>>  {
    let file = File::open("resources/input.txt")?;
    let buff = BufReader::new(file);

    let mut modules: HashMap<String, (ModuleType, Pulse, Vec<String>)> = HashMap::new();
    let mut conjunction_inputs: HashMap<String, HashMap<String, Pulse >> = HashMap::new();

    for line in buff.lines() {
        let line = line?;

        let mut split = line.split("->");
        let left = split.next().unwrap().trim();
        let module_type = if left.contains('%') {
            FlipFlop
        } else if left.contains('&') {
            Conjunction
        } else {
            Broadcast
        };
        let name = left
          .replace('%', "")
          .replace('&', "")
          .to_string();

        let right = split.next().unwrap().trim();
        let receivers = right.split(",").map(|s| s.trim().to_string()).collect::<Vec<String>>();
        modules.insert(name, (module_type, Low, receivers));
    }

    for (conjunction_name, (module_type,_ , _)) in modules.iter() {
        if module_type == &Conjunction {
            let mut inputs = HashMap::new();
            for (name, (_, _, outputs)) in modules.iter() {
                if outputs.contains(conjunction_name) {
                    inputs.insert(name.to_string(), Low);
                }
            }
            conjunction_inputs.insert(conjunction_name.clone(), inputs);
        }
    }

    for i in 0..100000 {
        button_pressed(&mut modules, &mut conjunction_inputs, i);
    }
    // get the period of the input of the conjunction before rx
    // vg: 3931
    // nb: 3851
    // vc: 3881
    // ls: 3943
    // least common multiple: 231657829136023

    return Ok(());
}

fn button_pressed(
    modules: &mut HashMap<String, (ModuleType, Pulse, Vec<String>)>,
    conjunction_inputs: &mut HashMap<String, HashMap<String, Pulse>>,
    i: i32
) -> bool {
    let broadcaster_name = "broadcaster".to_string();
    let broadcaster = modules.get(&broadcaster_name).unwrap();
    let receivers = &broadcaster.2.clone();
    let mut queue: Vec<(String, String, Pulse)> = receivers.iter().map(|r| (broadcaster_name.clone(), r.clone(), Low)).collect();

    propagate_pulse(modules, conjunction_inputs, &mut queue, i)
}

fn send_pulse(
    modules: &mut HashMap<String, (ModuleType, Pulse, Vec<String>)>,
    conjunction_inputs: &mut HashMap<String, HashMap<String, Pulse>>,
    from: &String,
    to: &String,
    pulse: &Pulse
) -> Option<Pulse> {
    let module = modules.get_mut(to).unwrap();
    match module.0 {
        FlipFlop => {
            return if pulse == &High {
                None
            } else {
                module.1.flip();
                Some(module.1)
            }
        }
        Conjunction => {
            let inputs = conjunction_inputs.get_mut(to).unwrap();
            let input = inputs.get_mut(from).unwrap();
            *input = pulse.clone();

            for value in inputs.values() {
                if value == &Low {
                    return Some(High);
                }
            }

            return Some(Low);
        }
        _ => {}
    }

    None
}

fn propagate_pulse(
    modules: &mut HashMap<String, (ModuleType, Pulse, Vec<String>)>,
    conjunction_inputs: &mut HashMap<String, HashMap<String, Pulse>>,
    queue: &mut Vec<(String, String, Pulse)>,
    i: i32
) -> bool {

    while !queue.is_empty() {
        let (from, to, pulse) = queue.remove(0);

        let receivers = match modules.get(&to) {
            None => continue,
            Some(a) => a.2.clone()
        };
        let propagated_pulse: Option<Pulse> = send_pulse(modules, conjunction_inputs, &from, &to, &pulse);
        let propagated_pulse = match propagated_pulse {
            Some(p) => p,
            None => continue,
        };

        if ["vg", "nb", "vc", "ls"].contains(&to.as_str()) {
            println!("{i} {}: {:?}", to, propagated_pulse);
        }

        for propagated_receiver in receivers {
            queue.push((to.clone(), propagated_receiver, propagated_pulse));
        }
    }

    false
}