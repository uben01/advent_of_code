use std::{error::Error, fs::File, io::{BufReader, BufRead}, usize};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("resources/input.txt")?;
    let buff = BufReader::new(file);

    let mut universe: Vec<Vec<char>> = vec![];

    let mut column_galaxy_count = vec![];
    let mut multiplied_row_ids = vec![];
    for (row_id , line) in buff.lines().enumerate() {
        let line = line?;

        let row: Vec<char> = line.chars().collect();

        if column_galaxy_count.len() == 0 {
            column_galaxy_count = vec![0; row.len()];
        }

        if !line.contains('#') {
           multiplied_row_ids.push(row_id);
        } else {
            for (idx, character) in row.iter().enumerate() {
                if character == &'#' {
                    column_galaxy_count[idx] += 1;
                }
            }
        }

        universe.push(row);
    }

    let mut multiplied_column_ids = vec![];
    for (column_id, count) in column_galaxy_count.iter().enumerate() {
        if count != &0 {
            continue
        }
        multiplied_column_ids.push(column_id);
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
    for from in &galaxy_coordinates {
        for to in galaxy_coordinates.iter().rev() {
            if from == to {
                break;
            }

            let rows = (usize::min(from.0, to.0), usize::max(from.0, to.0));
            let cols = (usize::min(from.1, to.1), usize::max(from.1, to.1));

            let mut multiplied_cells_through = 0;
            for multiplied_row in &multiplied_row_ids {
                if &rows.0 < multiplied_row && &rows.1 > multiplied_row {
                    multiplied_cells_through += 1;
                }
            }
            for multiplied_column in &multiplied_column_ids {
                if &cols.0 < multiplied_column && &cols.1 > multiplied_column {
                    multiplied_cells_through += 1;
                }
            }

            distances += rows.1 - rows.0;
            distances += cols.1 - cols.0;

            distances += 999_999 * multiplied_cells_through;
        }
    }

    println!("{}", distances);

    return Ok(());
}
