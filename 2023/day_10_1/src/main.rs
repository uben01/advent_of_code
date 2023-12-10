use std::{error::Error, fs::File, io::{BufReader, BufRead}};

fn main()  -> Result<(), Box<dyn Error>>  {
    let file = File::open("resources/input.txt")?;
    let buff = BufReader::new(file);

    let mut start_point = (0, 0);
    let mut map: Vec<Vec<char>> = vec![];
    for (idx, line) in buff.lines().enumerate() {
        let line = line?;

        if line.contains('S') {
            start_point = (idx as i32, line.find('S').unwrap() as i32);
        }

        let line_vec: Vec<char> = line.chars().collect();

        map.push(line_vec);
    }

    let directions = ['|', '-', 'L', 'J', '7', 'F'];

    let mut max_distance = 0;
    for direction in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let check_position = (start_point.0 + direction.0, start_point.1 + direction.1);
        let char_at_position = get_char_at_position(&map, check_position).unwrap_or('.');
        let new_direction = directions.iter().find(|&& x| x == char_at_position);
        match new_direction {
            None => continue,
            Some(_) => {
                max_distance = calculate_distances(&map, start_point, check_position, 1).unwrap();
                break;
            }
        }
    }

    println!("{}", max_distance / 2);

    Ok(())
}

fn calculate_distances(
    map: &Vec<Vec<char>>,
    from_position: (i32, i32),
    new_position: (i32, i32),
    depth: i32
) -> Result<i32, Box<dyn Error>> {
    let character = get_char_at_position(map, new_position).unwrap();
    if character == 'S' {
        return Ok(depth);
    }

    let newest_position = get_new_position_for_char(character, from_position, new_position).unwrap();

    Ok(calculate_distances(map, new_position, newest_position, depth + 1).unwrap())
}

fn get_char_at_position(map: &Vec<Vec<char>>, direction: (i32, i32)) -> Result<char, Box<dyn Error>> {
    let character = *map.get(direction.0 as usize).unwrap().get(direction.1 as usize).unwrap();

    Ok(character)
}

fn get_new_position_for_char(
    character: char,
    from_position: (i32, i32),
    new_position: (i32, i32)
) -> Result<(i32, i32), Box<dyn Error>> {
    let from_top: bool = from_position.0 < new_position.0;
    let from_left: bool = from_position.1 < new_position.1;
    let from_right: bool = from_position.1 > new_position.1;

    match character {
        '|' => {
            return if from_top {
                Ok((new_position.0 + 1, new_position.1))
            } else {
                Ok((new_position.0 - 1, new_position.1))
            }
        },
        '-' => {
            return if from_left {
                Ok((new_position.0, new_position.1 + 1))
            } else {
                Ok((new_position.0, new_position.1 - 1))
            }
        },
        'L' => {
            return if from_top {
                Ok((new_position.0, new_position.1 + 1))
            } else {
                Ok((new_position.0 - 1, new_position.1))
            }
        },
        'J' => {
            return if from_top {
                Ok((new_position.0, new_position.1 - 1))
            } else {
                Ok((new_position.0 - 1, new_position.1))
            }
        },
        '7' => {
            return if from_left {
                Ok((new_position.0 + 1, new_position.1))
            } else {
                Ok((new_position.0, new_position.1 - 1))
            }
        },
        'F' => {
            return if from_right {
                Ok((new_position.0 + 1, new_position.1))
            } else {
                Ok((new_position.0, new_position.1 + 1))
            }
        },
        _ => Err("Wrong character".into())
    }
}