use std::{fs::File, io::{BufReader, BufRead}, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("resources/input.txt")?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    let line = lines.next().unwrap();
    let line = line.unwrap();

    let patter_length = 14;
    let mut last_chars: Vec<char> = vec![];
    for (idx, character) in line.chars().enumerate() {
        if last_chars.len() < patter_length {
            last_chars.push(character);

            continue;
        }

        let mut is_there_duplicate = false;
        for i in 0..patter_length {
            for j in 0..patter_length {
               if last_chars[i] == last_chars[j] && i != j {
                   is_there_duplicate = true;
               } 
            }
        }
        if !is_there_duplicate {
            println!("{idx}");
            return Ok(());
        }
        
        for i in 1..patter_length {
            last_chars[i - 1] = last_chars[i];
        }

        last_chars[patter_length - 1] = character;
    }

    return Err("No pattern found".into());
}
