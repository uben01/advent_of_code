use std::{error::Error, fs::File, io::{BufReader, BufRead}};

#[derive(PartialEq, Clone)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

fn main()  -> Result<(), Box<dyn Error>>  {
    let file = File::open("resources/input.txt")?;
    let buff = BufReader::new(file);

    let mut map: Vec<Vec<char>> = vec![];
    for line in buff.lines() {
        let line = line?;

        let row: Vec<char> = line.chars().collect();
        map.push(row);
    }
    let mut energise_map: Vec<Vec<Vec<Direction>>> = vec![vec![vec![]; map[0].len()]; map.len()];
    follow_beam(&map, &mut energise_map, (0, -1), &Direction::Right);

    let mut count = 0;
    for row in &energise_map {
        for cell in row {
            if !cell.is_empty() {
                count += 1;
            }
            // print!("{}", if cell.is_empty() { '.' } else { '#' });
        }
        // println!();
    }

    println!("Count: {}", count);

    return Ok(());
}

fn follow_beam(
    map: &Vec<Vec<char>>,
    energise_map: &mut Vec<Vec<Vec<Direction>>>,
    from: (i32, i32),
    direction: &Direction)
{
    let next_coordinates = get_next_coordinates(from, &direction);

    if
        next_coordinates.0 < 0 ||
        next_coordinates.0 as usize >= map.len() ||
        next_coordinates.1 < 0 ||
        next_coordinates.1  as usize >= map[0].len()
    {
        return;
    }

    if energise_map[next_coordinates.0 as usize][next_coordinates.1 as usize].contains(&direction) {
        return;
    }

    energise_map[next_coordinates.0 as usize][next_coordinates.1 as usize].push(direction.clone());

    let next_directions = get_next_directions(map[next_coordinates.0 as usize][next_coordinates.1 as usize], &direction);

    for next_direction in next_directions {
        follow_beam(map, energise_map, (next_coordinates.0, next_coordinates.1), next_direction);
    }
}

fn get_next_coordinates(coordinates: (i32, i32), direction: &Direction) -> (i32, i32) {
    match direction {
        Direction::Right => (coordinates.0, coordinates.1 + 1),
        Direction::Down => (coordinates.0 + 1, coordinates.1),
        Direction::Left => (coordinates.0, coordinates.1 - 1),
        Direction::Up => (coordinates.0 - 1, coordinates.1),
    }
}

fn get_next_directions(char: char, direction: &Direction) -> Vec<&Direction> {
    match char {
        '.' => vec![direction],
        '|' => {
            match direction {
                Direction::Right | Direction::Left => vec![&Direction::Up, &Direction::Down],
                _ => vec![direction],
            }
        },
        '-' => {
            match direction {
                Direction::Up | Direction::Down => vec![&Direction::Left, &Direction::Right],
                _ => vec![direction],
            }
        },
        '\\' => {
            match direction {
                Direction::Right => vec![&Direction::Down],
                Direction::Down => vec![&Direction::Right],
                Direction::Left => vec![&Direction::Up],
                Direction::Up => vec![&Direction::Left],
            }
        },
        '/' => {
            match direction {
                Direction::Right => vec![&Direction::Up],
                Direction::Down => vec![&Direction::Left],
                Direction::Left => vec![&Direction::Down],
                Direction::Up => vec![&Direction::Right],
            }
        },
        _ => panic!("Unknown char: {}", char)
    }
}