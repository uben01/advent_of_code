use std::{error::Error, fs::File, io::{BufReader, BufRead}};

#[derive(PartialEq, Eq)]
enum RPS {
    ROCK,
    PAPER,
    SCISSOR
}

enum OUTCOME {
    WIN,
    LOSE,
    DRAW
}

impl RPS {
    fn from(s: String) -> RPS {
        if s == "A" || s == "X" {
           return RPS::ROCK; 
        } else if s == "B" || s == "Y" {
            return RPS::PAPER;
        }

        return RPS::SCISSOR;
    }

    fn play(&self, other: &RPS) -> OUTCOME {
        if self == other {
            return OUTCOME::DRAW;
        }
        match self {
            RPS::ROCK => {
                if *other == RPS::PAPER {
                    return OUTCOME::LOSE;
                }
                return OUTCOME::WIN;
            },
            RPS::PAPER => {
                if *other == RPS::ROCK {
                    return OUTCOME::WIN;
                }
                return OUTCOME::LOSE;
            },
            RPS::SCISSOR => {
                if *other == RPS::ROCK {
                    return OUTCOME::LOSE;
                }
                return OUTCOME::WIN;
            },
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

        let matchup: Vec<RPS> = line
            .split_whitespace()
            .map(|s| RPS::from(s.to_string()))
            .collect();

        let outcome = matchup[1].play(&matchup[0]);

        match matchup[1] {
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
