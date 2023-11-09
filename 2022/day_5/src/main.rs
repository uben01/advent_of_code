use std::{error::Error, fs::File, io::{BufReader, BufRead}};
use std::collections::HashMap;
use regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("resources/input.txt")?;
    let reader = BufReader::new(file);
    
    let mut setup_in_progress = true;
    let mut setup: HashMap<usize, Vec<char>> = HashMap::new();
    for line in reader.lines() {
        let line = line?;

        if line.clone().len() == 0 {
            setup_in_progress = false;
            continue;
        }

        if setup_in_progress {
            let mut crate_id = 1;
            let mut char_iter = line.chars();
            loop {
                match char_iter.next() { // [
                    Some(_) => {},
                    None => break,
                };
                let elem = char_iter.next().unwrap();
                char_iter.next(); // ]
                char_iter.next(); // space

                if elem == ' ' {
                    crate_id += 1;
                    continue;
                }

                if elem.is_numeric() {
                    break;
                }

                let optional_column = setup.get_mut(&crate_id);
                let nth_column;
                match optional_column {
                    None => {
                        let new_column: Vec<char> = Vec::new();

                        setup.insert(crate_id, new_column);

                        nth_column = setup.get_mut(&crate_id).unwrap();
                    },
                    Some(a) => {
                        nth_column = a;
                    }
                }

                nth_column.push(elem);

                crate_id += 1;
            }
        } else {
            let re = Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
            let result = re.captures(&line).unwrap();

            let number: usize = result[1].to_string().parse()?;
            let from: usize = result[2].to_string().parse()?;
            let to: usize = result[3].to_string().parse()?;

            for _ in 0..number {
                let from_column = setup.get_mut(&from).unwrap();
                let element = from_column.remove(0);
                let to_column = setup.get_mut(&to).unwrap();
                to_column.insert(0, element);
            }
        }

    }

    for i in 1..=9  {
        print!("{:?}", setup[&i][0]);
    }

    return Ok(());
}
