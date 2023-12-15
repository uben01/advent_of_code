use std::{error::Error, fs::File, io::{BufReader, BufRead}};

fn main()  -> Result<(), Box<dyn Error>>  {
    let file = File::open("resources/input.txt")?;
    let buff = BufReader::new(file);

    let mut sum = 0;
    for line in buff.lines() {
        let line = line?;

        let split = line.split(",");
        for block in split {
            let mut value = 0;
            for char in block.chars() {
                value += char as i32;
                value *= 17;
                value = value % 256;
            }
            sum += value;
        }
    }

    println!("The hash value is: {sum}");
    return Ok(());
}
