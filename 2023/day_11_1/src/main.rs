use std::{error::Error, fs::File, io::{BufReader, BufRead}};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("resources/input.txt")?;
    let buff = BufReader::new(file);

    let mut universe: Vec<Vec<char>> = vec![];
    let mut column_galaxy_count = vec![];
    for line in buff.lines() {
        let line = line?;

        let row: Vec<char> = line.chars().collect();

        if column_galaxy_count.len() == 0 {
            column_galaxy_count = vec![0; row.len()];
        }

        if !line.contains('#') {
            universe.push(row.clone());
        } else {
            for (idx, character) in row.iter().enumerate() {
                if character == &'#' {
                    column_galaxy_count[idx] += 1;
                }
            }
        }

        universe.push(row);
    }

    for row in universe.iter_mut() {
        let mut already_pushed = 0;
        for (column_id, count) in column_galaxy_count.iter().enumerate() {
            if count != &0 {
                continue
            }

            row.insert(column_id + already_pushed, '.');
            already_pushed += 1;
        }
    }

    let mut galaxy_coordinates: Vec<(usize, usize)> = vec![];

    for (row_id, galaxy) in universe.iter().enumerate() {
        for (column_id, character) in galaxy.iter().enumerate() {
            if character == &'#' {
                galaxy_coordinates.push((row_id, column_id));
            }
        }
    }

    let mut distances = 0;
    for (idx, from) in galaxy_coordinates.iter().enumerate() {
        for to in galaxy_coordinates.iter().rev() {
            if from == to {
                break;
            }
            let mut distance = 0;
            distance += i32::abs(from.0 as i32 - to.0 as i32);
            distance += i32::abs(from.1 as i32 - to.1 as i32);

            // println!("{idx}: {:?}, {:?}", from, to);
            // println!("{distance}");
            // println!();
            distances += distance;
        }
    }

    println!("{}", distances);

    return Ok(());
}
