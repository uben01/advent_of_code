use std::{error::Error, fs::File, io::{BufReader, BufRead}};

#[derive(PartialEq, Eq, Clone)]
enum RPS {
    ROCK,
    PAPER,
    SCISSOR
}

#[derive(PartialEq, Eq)]
enum OUTCOME {
    WIN,
    LOSE,
    DRAW
}

impl OUTCOME {
     fn from(s: String) -> OUTCOME {
        if s == "X" {
           return OUTCOME::LOSE; 
        } else if s == "Y" {
            return OUTCOME::DRAW;
        }

        return OUTCOME::WIN;
    }
}

impl RPS {
    fn from(s: String) -> RPS {
        if s == "A" {
           return RPS::ROCK; 
        } else if s == "B" {
            return RPS::PAPER;
        }

        return RPS::SCISSOR;
    }

    fn play(&self, outcome: &OUTCOME) -> RPS {
        match outcome {
            OUTCOME::WIN => {
                match *self {
                    RPS::ROCK => {return RPS::PAPER},
                    RPS::PAPER => {return RPS::SCISSOR},
                    RPS::SCISSOR => {return RPS::ROCK},
                }
            },
            OUTCOME::LOSE => {
                 match *self {
                    RPS::ROCK => {return RPS::SCISSOR},
                    RPS::PAPER => {return RPS::ROCK},
                    RPS::SCISSOR => {return RPS::PAPER},
                }

            },
            OUTCOME::DRAW => {return self.clone();}
        }
    } 
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("resources/input.txt")?;
    let buff = BufReader::new(file);

    let mut score = 0;
    for line in buff.lines() {
        let line = line?;
        if line.len() == 0 {
            continue;
        }
        
        let mut line_iter = line.split_whitespace();
        let opponent = RPS::from(line_iter.next().ok_or("no opponent choice")?.to_string());
        let outcome = OUTCOME::from(line_iter.next().ok_or("no outcome")?.to_string());

        let my_choice = opponent.play(&outcome);

        match my_choice {
            RPS::ROCK => score += 1,
            RPS::PAPER => score += 2,
            RPS::SCISSOR => score += 3,
        }

        match outcome {
            OUTCOME::WIN => score += 6,
            OUTCOME::LOSE => {},
            OUTCOME::DRAW => score += 3,
        }
    }
    println!("{score}");

    return Ok(());
}
