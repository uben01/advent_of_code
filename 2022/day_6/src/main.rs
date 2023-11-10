use std::{fs::File, io::{BufReader, BufRead}, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("resources/input.txt")?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    let line = lines.next().unwrap();
    let line = line.unwrap();

    let mut last_chars: Vec<char> = vec![];
    for (idx, character) in line.chars().enumerate() {
        if last_chars.len() < 4 {
            last_chars.push(character);

            continue;
        }

        let mut is_there_duplicate = false;
        for i in 0..4 {
            for j in 0..4 {
               if last_chars[i] == last_chars[j] && i != j {
                   is_there_duplicate = true;
               } 
            }
        }
        if !is_there_duplicate {
            println!("{idx}");
            return Ok(());
        }
        
        for i in 1..4 {
            last_chars[i - 1] = last_chars[i];
        }

        last_chars[3] = character;
    }

    return Err("No pattern found".into());
}
