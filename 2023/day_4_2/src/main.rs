use std::{error::Error, fs::File, io::{BufReader, BufRead}};

use regex::Regex;

fn main()  -> Result<(), Box<dyn Error>>  {
    let file = File::open("resources/input.txt")?;
    let buff = BufReader::new(file);

    let number_matcher = Regex::new(r"(\d+)").unwrap();

    let mut won_card_ids: Vec<usize> = vec![];
    for (idx, line) in buff.lines().enumerate() {
        won_card_ids.push(idx);

        let line = line?;

        let mut winning_numbers: Vec<i32> = vec![];
        let mut my_numbers: Vec<i32> = vec![];

        let line = line.split(":").last().unwrap();

        let split_line: Vec<&str> = line.split('|').collect();
        let left_side = split_line.get(0).unwrap();
        let right_side = split_line.get(1).unwrap();

        for capture in number_matcher.captures_iter(&left_side) {
            winning_numbers.push(capture[1].parse().unwrap());
        } 
        for capture in number_matcher.captures_iter(&right_side) {
            my_numbers.push(capture[1].parse().unwrap());
        }

        let my_card_count = won_card_ids.iter().fold(0, |acc, &x| {
            acc + if x == idx { 1 } else { 0 }
        });

        let mut match_count = 0;
        for number in my_numbers {
            if winning_numbers.contains(&number) {
                match_count += 1;
            }
        }
        
        for i in 1..=match_count {
            for _ in 0..my_card_count {
                won_card_ids.push(idx + i);
            }
        }
    }
    println!("The sum is: {:?}", won_card_ids.len());
    return Ok(());
}
