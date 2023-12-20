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

    let mut high = 0;
    let mut low = 0;
    for _ in 0..1000 {
        let (partial_high, partial_low) = button_pressed(&mut modules, &mut conjunction_inputs);
        high += partial_high;
        low += partial_low + 1;

        // println!("high: {partial_high}, low: {partial_low}");
    }

    println!("Result: {:?}", high * low);

    return Ok(());
}

fn button_pressed(
    modules: &mut HashMap<String, (ModuleType, Pulse, Vec<String>)>,
    conjunction_inputs: &mut HashMap<String, HashMap<String, Pulse>>,
) -> (usize, usize) {
    let broadcaster_name = "broadcaster".to_string();
    let broadcaster = modules.get(&broadcaster_name).unwrap();
    let receivers = &broadcaster.2.clone();
    let mut queue: Vec<(String, String, Pulse)> = receivers.iter().map(|r| (broadcaster_name.clone(), r.clone(), Low)).collect();

    let mut high = 0;
    let mut low = 0;

    // add result of propagation to high and low
    let (partial_high, partial_low) = propagate_pulse(modules, conjunction_inputs, &mut queue);
    high += partial_high;
    low += partial_low;

    (high, low)
}

fn send_pulse(
    modules: &mut HashMap<String, (ModuleType, Pulse, Vec<String>)>,
    conjunction_inputs: &mut HashMap<String, HashMap<String, Pulse>>,
    name: &String,
    from: &String,
    pulse: &Pulse
) -> Option<Pulse> {
    let module = modules.get_mut(name).unwrap();
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
            let inputs = conjunction_inputs.get_mut(name).unwrap();
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
    queue: &mut Vec<(String, String, Pulse)>
) -> (usize, usize) {
    let mut pulses_sent = (0, 0); // high, low

    while !queue.is_empty() {
        let (from, to, pulse) = queue.remove(0);
        // println!("{from} -{:?}-> {to}", pulse);

        match &pulse {
            High => pulses_sent.0 += 1,
            Low => pulses_sent.1 += 1,
        }

        let receivers = match modules.get(&to) {
            None => continue,
            Some(a) => a.2.clone()
        };

        let propagated_pulse: Option<Pulse> = send_pulse(modules, conjunction_inputs, &to, &from, &pulse);
        let propagated_pulse = match propagated_pulse {
            Some(p) => p,
            None => continue,
        };
        for propagated_receiver in receivers {
            queue.push((to.clone(), propagated_receiver, propagated_pulse));
        }
    }

    pulses_sent
}