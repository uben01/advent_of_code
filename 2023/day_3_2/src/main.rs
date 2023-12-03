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
                    row.push(Blob::create(idx, character.to_string(), character.is_numeric()));
                } else {
                    if character.is_numeric() && last.is_numeric() {
                        row.last_mut().unwrap().content.push(character);
                    } else {
                        row.push(Blob::create(idx, character.to_string(), character.is_numeric()));
                    }
                }
            }
            last = character;
        }
        table.push(row);
    }

    let mut sum: i32 = 0;
    for (row_id, row) in table.iter().enumerate() {
        'cell_loop: for (column_id, cell) in row.iter().enumerate() {
            if cell.numeric {
                continue;
            }

            let mut gears = vec![];

            // left
            if column_id != 0 {
                let left = row.get(column_id - 1).unwrap();
                if left.start + left.content.len() == cell.start {
                    gears.push(left);
                }
            }
            // right
            if column_id != row.len() - 1 {
                let right = row.get(column_id + 1).unwrap();
                if right.start == cell.start + 1 {
                    gears.push(right);
                }
            }
            // top
            if row_id != 0 {
                let top = table.get(row_id -1).unwrap();
                for top_cell in top {
                    if !top_cell.numeric {
                        continue;
                    }
                    let top_cell_right = top_cell.start + top_cell.content.len();

                    if top_cell.start <= cell.start + 1 && top_cell_right >= cell.start {
                        gears.push(top_cell);
                    }
                }
            }
            
            // bottom
            if row_id != table.len() - 1 {
                let bottom = table.get(row_id + 1).unwrap();
                for bottom_cell in bottom {
                    if !bottom_cell.numeric {
                        continue;
                    }
                    let bottom_cell_right = bottom_cell.start + bottom_cell.content.len();

                    if bottom_cell.start <= cell.start + 1 && bottom_cell_right >= cell.start {
                        gears.push(bottom_cell);
                    }
                }
            }

            if gears.len() == 2 {
                sum += gears[0].content.parse::<i32>().unwrap() * gears[1].content.parse::<i32>().unwrap();
            }
        }
    }

    println!("The sum is: {sum}");

    return Ok(());
}
