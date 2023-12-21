use std::{error::Error, fs::File, io::{BufReader, BufRead}};
use std::collections::HashSet;

enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn get_directions() -> Vec<Direction> {
        vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right]
    }

    fn displace(self, coordinate: &(isize, isize)) -> (isize, isize) {
        return match self {
            Direction::Up => {
                (coordinate.0 - 1, coordinate.1)
            },
            Direction::Down => {
                (coordinate.0 + 1, coordinate.1)
            },
            Direction::Left => {
                (coordinate.0, coordinate.1 - 1)
            },
            Direction::Right => {
                (coordinate.0, coordinate.1 + 1)
            },
        }
    }
}

fn main()  -> Result<(), Box<dyn Error>>  {
    let file = File::open("resources/input.txt")?;
    let buff = BufReader::new(file);

    let map: Vec<Vec<char>> = buff.lines().map(|line| line.unwrap().chars().collect()).collect();

    let mut start: (isize, isize) = (0, 0); // (row, column)
    for (y, row) in map.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if *col == 'S' {
                start = (x as isize, y as isize);
                break;
            }
        }
    }

    let results: Vec<String> = [0, 1, 2]
      .iter()
      .map(|i| find_final_places(map.clone(), start, 65 + 131 * *i))
      .map(|i| i.to_string())
      .collect();

    println!("{{{}}}", results.join(", "));

    // a = 15350
    // b = 15465
    // c = 3885
    // n = 202300
    // result = (a * n^2) + (b * n) + c
    // result = 628206330073385

    return Ok(());
}

fn find_final_places(map: Vec<Vec<char>>, start: (isize, isize), step_count: usize) -> usize {
    let mut starting_places = vec![HashSet::new(); step_count + 1];
    starting_places[0].insert(start);
    let width = map[0].len();
    let height = map.len();

    for nth_step in 0..step_count {
        let starting_places_for_round = starting_places[nth_step].clone();
        for starting_place in starting_places_for_round.iter() {
            for direction in Direction::get_directions() {
                let new_place = direction.displace(starting_place);
                let new_projected_place = find_coordinate_on_map(new_place, width, height);
                if map[new_projected_place.0][new_projected_place.1] == '#' {
                    continue;
                }
                starting_places[nth_step + 1].insert(new_place);
            }
        }
    }

    starting_places[step_count].len()
}

fn find_coordinate_on_map(coordinates: (isize, isize), width: usize, height: usize) -> (usize, usize) {
    let x = ((coordinates.0 % width as isize) + width as isize) % width as isize;
    let y = ((coordinates.1 % height as isize) + height as isize) % height as isize;
    return (x as usize, y as usize);
}
