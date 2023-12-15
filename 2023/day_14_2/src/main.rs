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

        for (column_id, char) in line.chars().enumerate() {
            map.get_mut(column_id).unwrap().push(char);
        }
    }

    tilt_n_times(&mut map, 143);
    for _ in 0..(1000000000 - 143) % 14 {
        tilt_n_times(&mut map, 1);
    }
    let mut sum = 0;
    for column in &map {
        for (row_id, char) in column.iter().enumerate() {
            if *char == 'O' {
                sum += column.len() - row_id;
            }
        }
    }

    println!("Sum: {}", sum);
    return Ok(());
}

fn tilt_n_times(map: &mut Vec<Vec<char>>, times: usize) {
    for _ in 0..times {
        roll_north_or_south(map, true);
        roll_west_or_east(map, true);
        roll_north_or_south(map, false);
        roll_west_or_east(map, false);
    }
}

fn roll_north_or_south(map: &mut Vec<Vec<char>>, north: bool) {
    let row_edge = if north { 0 } else { map[0].len() - 1 };
    let prev_modifier = if north { -1 } else { 1 };

    loop {
        let mut changed = false;

        let range: Vec<usize> = if north {
            (0..map[0].len()).collect()
        } else {
            (0..map[0].len()).rev().collect()
        };
        for column_id in 0..map.len() {
            let column = map.get_mut(column_id).unwrap();

            let mut last = '.';

            for row_id in &range {
                let current = column[*row_id];

                if *row_id == row_edge || last != '.' || current != 'O' {
                    last = current;
                    continue;
                }

                let prev_position = column.get_mut((*row_id as i32 + prev_modifier) as usize).unwrap();
                *prev_position = 'O';
                let current_position = column.get_mut(*row_id).unwrap();
                *current_position = '.';

                last = '.';
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
}

fn roll_west_or_east(map: &mut Vec<Vec<char>>, west: bool) {
    let column_edge = if west { 0 } else { map.len() - 1 };
    let prev_modifier = if west { -1 } else { 1 };

    let range: Vec<usize> = if west {
        (0..map.len()).collect()
    } else {
        (0..map.len()).rev().collect()
    };
    loop {
        let mut changed = false;

        for row_id in 0..map[0].len() {
            let mut last = '.';

            for column_id in &range {
                let current = map[*column_id][row_id];

                if *column_id == column_edge || last != '.' || current != 'O' {
                    last = current;
                    continue;
                }

                let prev_position = map.get_mut((*column_id as i32 + prev_modifier) as usize).unwrap().get_mut(row_id).unwrap();
                *prev_position = 'O';
                let current_position = map.get_mut(*column_id).unwrap().get_mut(row_id).unwrap();
                *current_position = '.';

                last = '.';
                changed = true;
            }

        }
        if !changed {
            break;
        }
    }
}