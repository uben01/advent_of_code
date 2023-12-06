use std::{error::Error, fs::File, io::{BufReader, BufRead}};
use regex::Regex;

fn main()  -> Result<(), Box<dyn Error>>  {
    let file = File::open("resources/input.txt")?;
    let buff = BufReader::new(file);


    let multiple_spaces_pattern = Regex::new(r"\s+").unwrap();

    let mut lines = buff.lines().into_iter();
    
    let time = lines.next().unwrap()?;
    let time = multiple_spaces_pattern.replace_all(&time, "");

    let distance = lines.next().unwrap()?;
    let distance = multiple_spaces_pattern.replace_all(&distance, "");
    
    let mut time = time.split(":");
    let mut distance = distance.split(":");

    let mut product = 1;
    time.next(); distance.next();
    let time: i64 = time.next().unwrap().parse().unwrap();
    let distance: i64 = distance.next().unwrap().parse().unwrap();

    let mut count = 0;
    for hold_duration in 0..time {
        let distance_traveled = (time - hold_duration) * hold_duration;
        if distance_traveled > distance {
            count += 1;
        }
    }
    product *= count;


    println!("{product}");
    return Ok(());
}
