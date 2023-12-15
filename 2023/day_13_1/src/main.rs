use std::{error::Error, fs::File, io::{BufReader, BufRead}};

fn main()  -> Result<(), Box<dyn Error>>  {
    let file = File::open("resources/input.txt")?;
    let buff = BufReader::new(file);

    let mut block: Vec<Vec<char>> = vec![];

    let mut a = 0;
    let mut b = 0;

    for line in buff.lines() {
        let line = line?;
        if line.is_empty() {
            let loc_a = find_reflection_axis_vertical(&block).unwrap_or(0);
            let loc_b = find_reflection_axis_horizontal(&block).unwrap_or(0);

            println!("a: {}, b: {}", loc_a, loc_b);

            a += loc_a;
            b += loc_b;

            // do the math
            block = vec![];
            continue;
        }

        block.push(line.chars().collect());
    }
    a += find_reflection_axis_vertical(&block).unwrap_or(0);
    b += find_reflection_axis_horizontal(&block).unwrap_or(0);

    let result = 100 * b + a;

    println!("Result: {}", result);

    return Ok(());
}

fn find_reflection_axis_vertical(block: &Vec<Vec<char>>) -> Option<usize> {
    let mut possible_axis: Vec<usize> = vec![];

    for (row_id, row) in block.iter().enumerate() {
        if row_id == 0 {
            for axis in 1..row.len() {
                if validate_axis(row, axis) {
                    possible_axis.push(axis);
                }
            }

            continue;
        }
        remove_invalid_axis(row, &mut possible_axis);
    }
    return possible_axis.pop();
}

fn find_reflection_axis_horizontal(block: &Vec<Vec<char>>) -> Option<usize> {
    let mut possible_axis: Vec<usize> = vec![];

    for column_id in 0..block[0].len() {
        let mut column: Vec<char> = vec![];
        for row in block.iter() {
            column.push(row[column_id]);
        }

        if column_id == 0 {
            for axis in 1..column.len() {
                if validate_axis(&column, axis) {
                    possible_axis.push(axis);
                }
            }

            continue;
        }
        remove_invalid_axis(&column, &mut possible_axis);
    }
    return possible_axis.pop();
}

fn validate_axis(row: &Vec<char>, axis: usize) -> bool {
    let split = row.split_at(axis);
    let mut left = split.0.iter().rev();
    let mut right = split.1.iter();

    while let (Some(left_char), Some(right_char)) = (left.next(), right.next()) {
        if left_char != right_char {
            return false;
        }
    }

    return true;
}

fn remove_invalid_axis(row: &Vec<char>, axis: &mut Vec<usize>) {
    let mut to_be_removed = vec![];
    for (id, axis) in axis.iter().enumerate() {
        if !validate_axis(row, *axis) {
            to_be_removed.push(id);
        }
    }
    for i in to_be_removed.iter().rev() {
        axis.remove(*i);
    }
}