use std::{error::Error, fs::File, io::{BufReader, BufRead}};
use std::collections::HashMap;

fn main()  -> Result<(), Box<dyn Error>>  {
    let file = File::open("resources/input.txt")?;
    let buff = BufReader::new(file);

    let mut block: Vec<Vec<char>> = vec![];

    let mut a = 0;
    let mut b = 0;

    for line in buff.lines() {
        let line = line?;
        if line.is_empty() {
            let (loc_a, loc_b) = fix_smudge_and_try(&mut block);
            let loc_a = loc_a.unwrap_or(0);
            let loc_b = loc_b.unwrap_or(0);

            println!("a: {}, b: {}", loc_a, loc_b);

            a += loc_a;
            b += loc_b;

            block = vec![];
            continue;
        }

        block.push(line.chars().collect());
    }

    let (loc_a, loc_b) = fix_smudge_and_try(&mut block);
    let loc_a = loc_a.unwrap_or(0);
    let loc_b = loc_b.unwrap_or(0);

    println!("a: {}, b: {}", loc_a, loc_b);

    a += loc_a;
    b += loc_b;

    let result = 100 * b + a;

    println!("Result: {}", result);

    return Ok(());
}

fn fix_smudge_and_try(block: &mut Vec<Vec<char>>) -> (Option<usize>, Option<usize>) {
    let vertical = find_reflection_axis_vertical(block);
    let horizontal = find_reflection_axis_horizontal(block);

    return (vertical, horizontal);
}

fn find_reflection_axis_vertical(block: &Vec<Vec<char>>) -> Option<usize> {
    let mut possible_axis: HashMap<usize, usize> = HashMap::new();

    for (row_id, row) in block.iter().enumerate() {
        if row_id == 0 {
            for axis in 1..row.len() {
                let error_count_for_axis = get_error_count_for_axis(row, &axis);
                if error_count_for_axis <= 1 {
                    possible_axis.insert(axis, error_count_for_axis);
                }
            }

            continue;
        }
        increase_invalid_axis(row, &mut possible_axis);
    }

    remove_invalid_axis(&mut possible_axis);

    if possible_axis.is_empty() {
        return None;
    }

    return Some(*possible_axis.iter().next().unwrap().0);
}

fn remove_invalid_axis(axis: &mut HashMap<usize, usize>) {
    let mut invalid_axis: Vec<usize> = vec![];

    for (id, error_count) in axis.iter() {
    if *error_count != 1 {
            invalid_axis.push(*id);
        }
    }

    for id in invalid_axis {
        axis.remove(&id);
    }
}

fn find_reflection_axis_horizontal(block: &Vec<Vec<char>>) -> Option<usize> {
    let mut possible_axis: HashMap<usize, usize> = HashMap::new();

    for column_id in 0..block[0].len() {
        let mut column: Vec<char> = vec![];
        for row in block.iter() {
            column.push(row[column_id]);
        }

        if column_id == 0 {
            for axis in 1..column.len() {
                let error_count_for_axis = get_error_count_for_axis(&column, &axis);
                if error_count_for_axis <= 1 {
                    possible_axis.insert(axis, error_count_for_axis);
                }
            }

            continue;
        }
        increase_invalid_axis(&column, &mut possible_axis);
    }

    remove_invalid_axis(&mut possible_axis);

    if possible_axis.is_empty() {
        return None;
    }

    return Some(*possible_axis.iter().next().unwrap().0);
}

fn get_error_count_for_axis(row: &Vec<char>, axis: &usize) -> usize {
    let split = row.split_at(*axis);
    let mut left = split.0.iter().rev();
    let mut right = split.1.iter();

    let mut error_count = 0;
    while let (Some(left_char), Some(right_char)) = (left.next(), right.next()) {
        if left_char != right_char {
            error_count += 1;
        }
    }

    return error_count;
}

fn increase_invalid_axis(row: &Vec<char>, axis: &mut HashMap<usize, usize>) {
    for id in axis.clone().keys() {
        let a = axis.get_mut(&id).unwrap();
        let error_count_for_axis = get_error_count_for_axis(row, id);

        *a += error_count_for_axis;
    }
}
