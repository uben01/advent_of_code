use std::{error::Error, fs::File, io::{BufReader, BufRead}};
use regex::Regex;

fn main()  -> Result<(), Box<dyn Error>>  {
    let file = File::open("resources/input.txt")?;
    let buff = BufReader::new(file);

    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let game_regex = Regex::new(r"Game (?<n>\d+): (?<r>.*)").unwrap();
    let semicolon_regex = Regex::new(r"(\d+ \w+,? ?)+").unwrap();
    let color_regex = Regex::new(r"(\d+) (\w+)").unwrap();

    let mut possible;

    let mut sum = 0;
    for line in buff.lines() {
        possible = true;
        let line = line?;

        let mut game_id: i32 = -1;
        let mut rest: String = "".to_string();

        if let Some(captures) = game_regex.captures(&line) {
            game_id = captures["n"].parse().unwrap();
            rest = captures["r"].to_string();
        }
        
        for captures in semicolon_regex.captures_iter(&rest) {
            let group = &captures[0].trim().replace(';', "");

            for colors in color_regex.captures_iter(&group) {
                let num: i32 = colors[1].trim().parse().unwrap();
                let color = colors[2].trim();

                if color == "red" {
                    if num > max_red {
                       possible = false; 
                    } 
                } else if color == "green" { 
                    if num > max_green {
                       possible = false; 
                    } 
                } else if color == "blue" {
                    if num > max_blue {
                       possible = false; 
                    } 
                }
            }
        }
        if possible {
            sum += game_id;
        }
    }
    println!("The sum is: {sum}");
    return Ok(());
}
