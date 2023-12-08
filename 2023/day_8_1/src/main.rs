use std::{error::Error, fs::File, io::{BufReader, BufRead}};
use std::collections::HashMap;

fn main()  -> Result<(), Box<dyn Error>>  {
    let file = File::open("resources/input.txt")?;
    let buff = BufReader::new(file);

    let mut left_right_instructions= vec![];
    let mut steps: HashMap<String, (String, String)> = HashMap::new();
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

        steps.insert(from, (to_left, to_right));
    }

    let instructions_length = left_right_instructions.clone().len();
    let mut step_count = 0;
    let mut starting_point = "AAA";
    loop {
        let next_possibilities = steps
            .get(starting_point)
            .unwrap();
        let nex_instruction = left_right_instructions
            .get(step_count % instructions_length)
            .unwrap();

        step_count += 1;

        if nex_instruction == &'L' {
            starting_point = &next_possibilities.0;
        } else {
            starting_point = &next_possibilities.1;
        }

        if starting_point == "ZZZ" {
            break;
        }
    }

    println!("Required step number was: {step_count}");
    return Ok(());
}
