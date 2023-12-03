use std::{error::Error, fs::File, io::{BufReader, BufRead}};

struct Blob {
    start: usize,
    content: String,
    numeric: bool
}

impl Blob {
    pub fn create(start: usize, content: String, numeric: bool) -> Blob {
        return Blob {
            start,
            content,
            numeric
        }
    }
}

fn main()  -> Result<(), Box<dyn Error>>  {
    let file = File::open("resources/input.txt")?;
    let buff = BufReader::new(file);
    
    let mut table: Vec<Vec<Blob>> = Vec::new();
    for line in buff.lines() {
        let mut row: Vec<Blob> = Vec::new();

        let mut last: char = '.';
        for (idx, character) in line?.chars().enumerate() {
            if character != '.' {
                let numeric = character.is_numeric();
                if !numeric && character != '*' {
                    continue;
                }

                if last == '.' {
                    row.push(Blob::create(idx, character.to_string(), numeric));
                } else {
                    if numeric && last.is_numeric() {
                        row.last_mut().unwrap().content.push(character);
                    } else {
                        row.push(Blob::create(idx, character.to_string(), numeric));
                    }
                }
            }
            last = character;
        }
        table.push(row);
    }

    let mut sum: i32 = 0;
    for (row_id, row) in table.iter().enumerate() {
        for (column_id, cell) in row.iter().enumerate() {
            if cell.numeric {
                continue;
            }

            let mut blobs_to_check = vec![];
            // left
            if column_id != 0 {
                blobs_to_check.push(row.get(column_id - 1).unwrap())
            }
            // right
            if column_id != row.len() - 1 {
                blobs_to_check.push(row.get(column_id + 1).unwrap());
            }
            // top
            if row_id != 0 {
                let top = table.get(row_id - 1).unwrap();
                for top_cell in top {
                    blobs_to_check.push(top_cell);
                }
            }
            
            // bottom
            if row_id != table.len() - 1 {
                let bottom = table.get(row_id + 1).unwrap();
                for bottom_cell in bottom {
                    blobs_to_check.push(bottom_cell);
                }
            }

            let mut gears = vec![];
            for blob in blobs_to_check {
                if check_cell(cell, blob) {
                    gears.push(blob.content.parse::<i32>().unwrap());
                }
            }

            if gears.len() == 2 {
                sum += gears[0] * gears[1];
            }
        }
    }

    println!("The sum is: {sum}");

    return Ok(());
}

fn check_cell(blob: &Blob, other: &Blob) -> bool {
    if !other.numeric {
        return false;
    }
    let other_cell_right = other.start + other.content.len();

    if other.start <= blob.start + 1 && other_cell_right >= blob.start {
        return true;
    }

    return false;
}