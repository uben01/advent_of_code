use std::{error::Error, fs::File, io::{BufReader, BufRead}};

fn main()  -> Result<(), Box<dyn Error>>  {
    let file = File::open("resources/input.txt")?;
    let buff = BufReader::new(file);

    let mut map: Vec<Vec<char>> = vec![];
    for (row_num, line) in buff.lines().enumerate() {
        let line = line?;

        if row_num == 0 {
            map = vec![vec![]; line.len()];
        }

        //collect columns in map
        for (column_id, char) in line.chars().enumerate() {
            map.get_mut(column_id).unwrap().push(char);
        }
    }
    loop {
        let mut changed = false;

        for column_id in 0..map.len() {
            let column = map.get_mut(column_id).unwrap();

            let mut last = '.';
            for row_id in 0..column.len() {
                let current = column[row_id];

                if row_id == 0 || last != '.' || current != 'O' {
                    last = column[row_id];
                    continue;
                }

                let prev_position = column.get_mut(row_id - 1).unwrap();
                *prev_position = 'O';
                let current_position = column.get_mut(row_id).unwrap();
                *current_position = '.';

                last = '.';
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }

    let mut sum = 0;
    for column in map.iter() {
        for (row_id, char) in column.iter().enumerate() {
            if *char == 'O' {
                sum += column.len() - row_id;
            }
        }
    }

    println!("Result: {:?}", sum);

    return Ok(());
}