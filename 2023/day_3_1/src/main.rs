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
            if !cell.numeric {
                continue;
            }
            let cell_right_edge = cell.start + cell.content.len();

            // left
            if column_id != 0 {
                let left = row.get(column_id - 1).unwrap();
                if left.start + 1 == cell.start {
                    sum += cell.content.parse::<i32>().unwrap();
                    continue 'cell_loop;
                }
            }
            // right
            if column_id != row.len() - 1 {
                let right = row.get(column_id + 1).unwrap();
                if right.start == cell_right_edge {
                    sum += cell.content.parse::<i32>().unwrap();
                    continue 'cell_loop;
                }
            }
            // top
            if row_id != 0 {
                let top = table.get(row_id -1).unwrap();
                for top_cell in top {
                    if top_cell.numeric {
                        continue;
                    }

                    if top_cell.start + 1 >= cell.start && top_cell.start <= cell_right_edge {
                        sum += cell.content.parse::<i32>().unwrap();
                        continue 'cell_loop;
                    }
                }
            }
            
            // bottom
            if row_id != table.len() - 1 {
                let bottom = table.get(row_id + 1).unwrap();
                for bottom_cell in bottom {
                    if bottom_cell.numeric {
                        continue;
                    }

                    if bottom_cell.start + 1 >= cell.start && bottom_cell.start <= cell_right_edge {
                        sum += cell.content.parse::<i32>().unwrap();
                        continue 'cell_loop;
                    }
                }
            }
        }
    }

    println!("The sum is: {sum}");

    return Ok(());
}
