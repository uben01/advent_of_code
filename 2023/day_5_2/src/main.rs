use std::{error::Error, fs::File, io::{BufReader, BufRead}};
use rayon::prelude::*;

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

struct SeedRange {
    start: i64,
    range: i64
}

impl SeedRange {
    fn create(start: i64, range: i64) -> SeedRange {
        SeedRange { start, range }
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("resources/input.txt")?;
    let buff = BufReader::new(file);

    let mut seeds: Vec<SeedRange> = vec![];
    let mut last_state: usize = 0;

    let mut converters: Vec<Vec<Converter>> = vec![];

    for (idx, line) in buff.lines().enumerate() {
        let mut line = line?;
        if idx == 0 {
            line = line.replace("seeds: ", "");
            let mut split_iter = line.split(' ');
            while let (Some(start), Some(range)) = (split_iter.next(), split_iter.next()) {
               seeds.push(SeedRange::create(start.parse().unwrap(), range.parse().unwrap())); 
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

    let minimal_result = seeds
        .into_par_iter()
        .map(|seed| process_range(seed, &converters))
        .reduce( || i64::MAX, i64::min);

    println!("Minimal result: {:}", minimal_result);
    return Ok(());
}

fn process_range(
    seed_range: SeedRange,
    converters: &Vec<Vec<Converter>>,
) -> i64 {
    let mut minimal_result = None;

    for j in seed_range.start..(seed_range.start + seed_range.range) {
        let mut result = j;
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

    return minimal_result.unwrap();
}