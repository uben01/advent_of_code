use std::{error::Error, fs::File, io::{BufReader, BufRead}};
use regex::Regex;

fn main()  -> Result<(), Box<dyn Error>>  {
    let file = File::open("resources/test.txt")?;
    let buff = BufReader::new(file);

    let game_regex = Regex::new(r"Game (?<n>\d+): (?<r>.*)").unwrap();
    let semicolon_regex = Regex::new(r"(\d+ \w+,? ?)+").unwrap();
    let color_regex = Regex::new(r"(\d+) (\w+)").unwrap();

    let mut sum = 0;
    for line in buff.lines() {
        let line = line?;

        let mut rest: String = "".to_string();

        if let Some(captures) = game_regex.captures(&line) {
            rest = captures["r"].to_string();
        }
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for captures in semicolon_regex.captures_iter(&rest) {
            let group = &captures[0].trim().replace(';', "");

            for colors in color_regex.captures_iter(&group) {
                let num: i32 = colors[1].trim().parse().unwrap();
                let color = colors[2].trim();

                if color == "red" {
                    if num > max_red {
                        max_red = num; 
                    } 
                } else if color == "green" { 
                    if num > max_green {
                        max_green = num; 
                    } 
                } else if color == "blue" {
                    if num > max_blue {
                        max_blue = num; 
                    } 
                }
            }
        }
        sum += max_red * max_green * max_blue;
    }
    println!("The sum is: {sum}");
    return Ok(());
}
