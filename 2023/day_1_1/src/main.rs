use std::{error::Error, fs::File, io::{BufReader, BufRead}};

fn main()  -> Result<(), Box<dyn Error>>  {
    let file = File::open("resources/input.txt")?;
    let buff = BufReader::new(file);

    let mut sum = 0;

    for line in buff.lines() {
        let line = line?;

        let mut first = None;
        let mut last = None;

        for char in line.chars() {
            if char.is_numeric() {
                if first.is_none() {
                   first = Some(char.to_digit(10).unwrap());
                } else {
                    last = Some(char.to_digit(10).unwrap());
                }
            }
        }
        if last.is_none() {
            last = first;
        }
        let number: String = format!("{}{}", first.unwrap(), last.unwrap());

        println!("{number}");

        let number = number.parse::<i32>().unwrap();
        sum += number;
    }
    println!("The sum is: {sum}");
    return Ok(());
}
