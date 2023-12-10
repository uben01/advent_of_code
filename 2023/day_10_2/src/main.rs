use std::{error::Error, fs::File, io::{BufReader, BufRead}};

fn main()  -> Result<(), Box<dyn Error>>  {
    let file = File::open("resources/input.txt")?;
    let buff = BufReader::new(file);

    let mut start_point = (0, 0);
    let mut map: Vec<Vec<char>> = vec![];
    let mut cycle_map: Vec<Vec<bool>> = vec![];

    for (idx, line) in buff.lines().enumerate() {
        let line = line?;

        if line.contains('S') {
            start_point = (idx as i32 + 1, line.find('S').unwrap() as i32);
        }

        let line_vec: Vec<char> = line.chars().collect();

        cycle_map.push(vec![false; line.len()]);

        map.push(line_vec);
    }

    map.insert(0, vec!['.'; map.get(0).unwrap().len()]);
    cycle_map.insert(0, vec![false; map.get(0).unwrap().len()]);

    let directions = vec!['|', '-', 'L', 'J', '7', 'F'];

    let mut right_directions: Vec<(i32, i32)> = vec![];
    for direction in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let check_position = (start_point.0 + direction.0, start_point.1 + direction.1);
        let char_at_position = get_char_at_position(&map, check_position).unwrap_or('.');
        let new_direction = directions.iter().find(|&& x| x == char_at_position);
        match new_direction {
            None => {continue;}
            Some(_) => {
                if right_directions.is_empty() {
                    walk_cycle(&map, cycle_map.as_mut(), start_point, check_position).unwrap();
                }
                right_directions.push(direction);
            }
        }
    }

    if right_directions[0] == (1, 0) {
        if right_directions[1] == (-1, 0) {
            set_character_at_position(&mut map, start_point, '|');
        } else if right_directions[1] == (0, 1) {
            set_character_at_position(&mut map, start_point, 'L');
        } else {
            set_character_at_position(&mut map, start_point, 'J');
        }
    } else if right_directions[0] == (-1, 0) {
        if right_directions[1] == (0, 1) {
            set_character_at_position(&mut map, start_point, 'F');
        } else {
            set_character_at_position(&mut map, start_point, '7');
        }
    } else {
        set_character_at_position(&mut map, start_point, '-');
    }


    let mut count = 0;
    let wall_types = vec!['F', 'J', '7', 'L', '-'];
    let lefties = ['J', '7'];
    let righties = ['F', 'L'];

    for row_id in 1..map.len() {
        let row = map.get(row_id).unwrap();
        for column_id in 0..row.len() {
            if get_visited_at_position(&cycle_map, (row_id as i32, column_id as i32)).unwrap() {
                continue;
            }

            let mut went_trough: Vec<char> = vec![];
            for i in 0..row_id {
                let is_cycle_part = get_visited_at_position(&cycle_map, (i as i32, column_id as i32)).unwrap();
                let current = get_char_at_position(&map, (i as i32, column_id as i32)).unwrap();

                if is_cycle_part {
                    if wall_types.contains(&current) {
                        went_trough.push(current);
                    }
                }
            }

            let lefts = went_trough.clone().iter().filter(|x| lefties.contains(x)).count();
            let right = went_trough.clone().iter().filter(|x| righties.contains(x)).count();
            let walls = went_trough.clone().iter().filter(|x| ['-'].contains(x)).count();

            let mut sum = walls;
            if lefts % 2 == 1 && right % 2 == 1 {
                sum += 1;
            }

            if sum % 2 != 0 {
                println!("{row_id}, {column_id}");
                count += 1;
            }
        }
    }

    println!("{count}");

    Ok(())
}

fn walk_cycle(
    map: &Vec<Vec<char>>,
    cycle_map: &mut Vec<Vec<bool>>,
    from_position: (i32, i32),
    new_position: (i32, i32),
) -> Result<(), Box<dyn Error>> {
    let character = get_char_at_position(map, new_position).unwrap();
    if character == 'S' {
        return Ok(());
    }

    set_visited_at_position(cycle_map, from_position);
    set_visited_at_position(cycle_map, new_position);

    let newest_position = get_new_position_for_char(character, from_position, new_position).unwrap();

    Ok(walk_cycle(map, cycle_map, new_position, newest_position).unwrap())
}

fn get_new_position_for_char(
    character: char,
    from_position: (i32, i32),
    new_position: (i32, i32)
) -> Result<(i32, i32), Box<dyn Error>>
{
    let from_top: bool = from_position.0 < new_position.0;
    let from_left: bool = from_position.1 < new_position.1;
    let from_right: bool = from_position.1 > new_position.1;

    match character {
        '|' => {
            if from_top {
                return Ok((new_position.0 + 1, new_position.1));
            } else {
                return Ok((new_position.0 - 1, new_position.1));
            }
        },
        '-' => {
            if from_left {
                return Ok((new_position.0, new_position.1 + 1));
            } else {
                return Ok((new_position.0, new_position.1 - 1));
            }
        },
        'L' => {
            if from_top {
                return Ok((new_position.0, new_position.1 + 1));
            } else {
                return Ok((new_position.0 - 1, new_position.1));
            }
        },
        'J' => {
            if from_top {
                return Ok((new_position.0, new_position.1 - 1));
            } else {
                return Ok((new_position.0 - 1, new_position.1));
            }
        },
        '7' => {
            if from_left {
                return Ok((new_position.0 + 1, new_position.1));
            } else {
                return Ok((new_position.0, new_position.1 - 1));
            }
        },
        'F' => {
            if from_right {
                return Ok((new_position.0 + 1, new_position.1));
            } else {
                return Ok((new_position.0, new_position.1 + 1));
            }
        },
        _ => Err("Wrong character".into()) }
}

fn get_char_at_position(map: &Vec<Vec<char>>, position: (i32, i32)) -> Option<char> {
    match map.get(position.0 as usize) {
        None => return None,
        Some(a) => {
            match a.get(position.1 as usize) {
                None => return None,
                Some(b) => Some(*b)
            }
        }
    }
}

fn get_visited_at_position(map: &Vec<Vec<bool>>, position: (i32, i32)) -> Option<bool> {
    match map.get(position.0 as usize) {
        None => return None,
        Some(a) => {
            match a.get(position.1 as usize) {
                None => return None,
                Some(b) => Some((*b).clone())
            }
        }
    }
}

fn set_visited_at_position(map: &mut Vec<Vec<bool>>, position: (i32, i32)) {
    let visited = map.get_mut(position.0 as usize).unwrap().get_mut(position.1 as usize).unwrap();
    *visited = true;
}

fn set_character_at_position(map: &mut Vec<Vec<char>>, position: (i32, i32), character: char) {
    let visited = map.get_mut(position.0 as usize).unwrap().get_mut(position.1 as usize).unwrap();
    *visited = character;
}