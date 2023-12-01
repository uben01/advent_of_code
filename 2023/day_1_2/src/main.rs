use std::{error::Error, fs::File, io::{BufReader, BufRead}};
use regex::Regex;

fn main()  -> Result<(), Box<dyn Error>>  {
    let numbers_strings = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let numbers_digits = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"];

    let mut sum = 0;

    let file = File::open("resources/input.txt")?;
    let buff = BufReader::new(file);
    for line in buff.lines() {
        let line = line?;

        let mut first = None;
        let mut last = None;
        let mut first_idx = None;
        let mut last_idx = None;
        
        for number_list in [&numbers_digits, &numbers_strings] {
            for (idx, value) in number_list.iter().enumerate() {
                let regex = Regex::new(value).unwrap();
                
                for capture in regex.find_iter(&line) {
                    let start_offset = capture.start();
                    if first_idx.is_none() || first_idx.unwrap() > start_offset {
                        first_idx = Some(start_offset);
                        first = Some(idx + 1);
                    }
                    if last_idx.is_none() || last_idx.unwrap() < start_offset {
                        last_idx = Some(start_offset);
                        last = Some(idx + 1);
                    }
                }
            }
        }
        
        if last == None {
            last = first;
        }

        let number = format!("{}{}", first.unwrap(), last.unwrap());
        let number = number.parse::<i32>().unwrap();
        sum += number;

    }
    println!("The sum is: {sum}");
    return Ok(());
}
