use std::{error::Error, fs::File, io::{BufReader, BufRead}};
use rayon::prelude::*;
use crate::Direction::{Down, Left, Right, Up};

#[derive(PartialEq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn get_next(&self, position: &(usize, usize)) -> Option<(usize, usize)> {
        if (position.0 == 0 && *self == Up) || (position.1 == 0 && *self == Right) {
            return None;
        }

        Some(match self {
            Up => (position.0 - 1, position.1),
            Down => (position.0 + 1, position.1),
            Left => (position.0, position.1 - 1),
            Right => (position.0, position.1 + 1)
        })
    }

    fn get_all() -> [Direction; 4] {
        [Up, Down, Left, Right]
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("resources/input.txt")?;
    let buff = BufReader::new(file);

    let map: Vec<Vec<char>> = buff
        .lines()
        .map(
            |line| line
                .unwrap()
                .chars()
                .collect()
        )
        .collect();

    let rows = map.len();
    let columns = map[0].len();

    let start = (0usize, map[0].iter().position(|&c| c == '.').unwrap());
    let finish = (rows - 1, map[columns - 1].iter().position(|&c| c == '.').unwrap());

    let mut distances = vec![vec![0; columns]; rows];

    let result = find_longest_hike(&map, &mut distances, &start, &finish);

    println!("Result: {}", result);

    return Ok(());
}

fn find_longest_hike(
    map: &Vec<Vec<char>>,
    distances: &mut Vec<Vec<usize>>,
    start: &(usize, usize),
    finish: &(usize, usize),
) -> usize {
    let rows = map.len();
    let columns = map[0].len();

    let mut current = start.clone();
    loop {
        let current_distance = distances[current.0][current.1];

        if current == *finish {
            return distances[current.0][current.1];
        }

        let mut found_paths = Vec::with_capacity(3);

        for direction in Direction::get_all() {
            let next = get_next(&map, &distances, &direction, &current, &rows, &columns);

            let Some(next) = next else { continue; };

            found_paths.push(next);
        }
        if found_paths.len() == 0 {
            return 0;
        }
        if found_paths.len() > 1 {
            let max_path_length = found_paths.par_iter().map(|found_path| {
                let mut distances = distances.clone();
                let found_path = found_path.clone();
                distances[found_path.0][found_path.1] = current_distance + 1;

                find_longest_hike(map, &mut distances, &found_path, finish)
            }).max();

            return max_path_length.unwrap();
        }

        current = found_paths[0];
        distances[current.0][current.1] = current_distance + 1;
    }
}

fn get_next(
    map: &Vec<Vec<char>>,
    distances: &Vec<Vec<usize>>,
    direction: &Direction,
    current: &(usize, usize),
    rows: &usize,
    columns: &usize
) -> Option<(usize, usize)> {
    let next = direction.get_next(&current);
    if next.is_none() {
        return None;
    }
    let next = next.unwrap();
    if next.0 >= *rows || next.1 >= *columns {
        return None;
    }
    if distances[next.0][next.1] != 0 {
        return None;
    }
    if map[next.0][next.1] == '#' {
        return None;
    }
    // if *direction == Up && map[next.0][next.1] == 'v' ||
    //   *direction == Down && map[next.0][next.1] == '^' ||
    //   *direction == Left && map[next.0][next.1] == '>' ||
    //   *direction == Right && map[next.0][next.1] == '<' {
    //     return None;
    // }

    Some(next)
}