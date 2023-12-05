use std::{error::Error, fs::File, io::{BufReader, BufRead}};

struct Converter {
    destination: i64,
    source: i64,
    range: i64,
}

impl Converter {
    fn create(destination: i64, source: i64, range: i64) -> Converter {
        Converter {
            destination,
            source,
            range,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("resources/input.txt")?;
    let buff = BufReader::new(file);

    let mut seeds: Vec<String> = vec![];
    let mut last_state: usize = 0;

    let mut converters: Vec<Vec<Converter>> = vec![];

    for (idx, line) in buff.lines().enumerate() {
        let mut line = line?;
        if idx == 0 {
            line = line.replace("seeds: ", "");
            for seed in line.split(' ') {
                seeds.push(seed.to_string());
            }
            continue;
        }

        if line.len() == 0 {
            continue;
        }

        if !line.chars().nth(0).unwrap().is_numeric() {
            last_state += 1;
            converters.push(vec![]);
            continue;
        }

        let split: Vec<&str> = line.split(' ').collect();
        let parsed_split = split.iter();
        let parsed_split: Vec<i64> = parsed_split.map(|i| i.parse().unwrap()).collect();

        converters.get_mut(last_state - 1).unwrap().push(
            Converter::create(
                *parsed_split.get(0).unwrap(),
                *parsed_split.get(1).unwrap(),
                *parsed_split.get(2).unwrap(),
            )
        );
    }

    let mut minimal_result = None;
    for seed in seeds {
        let seed: i64 = seed.parse().unwrap();

        let mut result = seed;
        for i in 0..7 {
            let stage = converters.get(i).unwrap();

            for converter in stage {
                let difference = result - converter.source;

                if difference >= 0 && difference < converter.range {
                    result = converter.destination + difference;

                    break;
                }
            }
        }

        if minimal_result.is_none() || minimal_result.unwrap() > result {
            minimal_result = Some(result);
        }

    }
    
    println!("Minimal result: {:}", minimal_result.unwrap());
    return Ok(());
}
