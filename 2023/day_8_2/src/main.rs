use std::{error::Error, fs::File, io::{BufReader, BufRead}};
use std::collections::HashMap;
use num::integer::lcm;

fn main()  -> Result<(), Box<dyn Error>>  {
    let file = File::open("resources/input.txt")?;
    let buff = BufReader::new(file);

    let mut left_right_instructions= vec![];
    let mut steps: HashMap<String, (String, String)> = HashMap::new();
    let mut starting_points = vec![];
    for (idx, line) in buff.lines().enumerate() {
        if idx == 0 {
            left_right_instructions = line?.chars().collect();
            continue;
        }
        if idx == 1 {
            continue;
        }

        let line = line.unwrap();
        let mut split = line.split(" = ");
        let from = split.next().unwrap().to_string();
        let to = split.next().unwrap()
            .replace('(', "")
            .replace(')', "");
        let mut split = to.split(", ");
        let to_left = split.next().unwrap().trim().to_string();
        let to_right = split.next().unwrap().trim().to_string();

        steps.insert(from.to_string(), (to_left, to_right));

        if from.clone().chars().into_iter().nth(2).unwrap() == 'A' {
            starting_points.push(from);
        }
    }

    let instructions_length = left_right_instructions.clone().len();
    let mut step_counts = vec![];

    for starting_point in starting_points {
        let mut step_count = 0;

        let mut starting_point = starting_point;
        loop {
            let next_possibilities = steps
                .get(starting_point.as_str())
                .unwrap();
            let nex_instruction = left_right_instructions
                .get(step_count % instructions_length)
                .unwrap();

            step_count += 1;

            if nex_instruction == &'L' {
                starting_point = String::from(next_possibilities.0.as_str());
            } else {
                starting_point = String::from(next_possibilities.1.as_str());
            }

            if starting_point.chars().nth(2).unwrap() == 'Z' {
                break;
            }
        }

        step_counts.push(step_count as u64);
    }

    let lcm = step_counts.iter().fold(step_counts[0], |acc, &x| lcm(acc, x));

    println!("Required step number was: {lcm}");
    return Ok(());
}
