use std::{error::Error, io::BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let file = std::fs::File::open("./resources/input.txt")?;
    let reader = std::io::BufReader::new(file);

    let mut max = -1;
    let mut current = 0;
    for line in reader.lines() {
        let value = line?.to_string();
        let value = value.trim();
        if value.len() == 0 {
            if current > max {
               max = current; 
            }

            current = 0;
            continue;
        }
        
        current += value.parse::<i32>()?;
    }
    println!("{max}");

    return Ok(());
}
