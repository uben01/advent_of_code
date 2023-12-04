use std::{error::Error, fs::File, io::{BufReader, BufRead}};

use regex::Regex;

fn main()  -> Result<(), Box<dyn Error>>  {
    let file = File::open("resources/input.txt")?;
    let buff = BufReader::new(file);

    let mut sum = 0;

    let card_matcher = Regex::new(r"Card\s*\d+:(.*)").unwrap();
    let winning_matcher = Regex::new(r"(.*)\|(.*)").unwrap();
    let number_matcher = Regex::new(r"(\d+)").unwrap();
    for line in buff.lines() {
        let line = line?;

        let mut winning_numbers: Vec<i32> = vec![];
        let mut my_numbers: Vec<i32> = vec![];

        let captures = card_matcher.captures(&line).unwrap();
        let line = &captures[1];

        let captures = winning_matcher.captures(&line).unwrap();
        let left_side = &captures[1];
        let right_side = &captures[2];

        for capture in number_matcher.captures_iter(&left_side) {
            winning_numbers.push(capture[1].parse().unwrap());
        } 
        for capture in number_matcher.captures_iter(&right_side) {
            my_numbers.push(capture[1].parse().unwrap());
        }

        println!("-----");
        let mut match_count = 0;
        for number in my_numbers {
            if winning_numbers.contains(&number) {
                println!("{number}");
                match_count += 1;
            }
        }

        if match_count > 0 {
            sum += (2 as i32).pow(match_count - 1);
        }

    }
    println!("The sum is: {sum}");
    return Ok(());
}
