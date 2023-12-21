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

    fn displace(self, coordinate: &(usize, usize), map_dimensions: (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Direction::Up => if coordinate.0 > 0 {
                return Some((coordinate.0 - 1, coordinate.1));
            },
            Direction::Down => if coordinate.0 < map_dimensions.0 - 1 {
                return Some((coordinate.0 + 1, coordinate.1));
            },
            Direction::Left => if coordinate.1 > 0 {
                return Some((coordinate.0, coordinate.1 - 1));
            },
            Direction::Right => if coordinate.1 < map_dimensions.1 - 1 {
                return Some((coordinate.0, coordinate.1 + 1));
            },
        }

        return None;
    }
}

fn main()  -> Result<(), Box<dyn Error>>  {
    let file = File::open("resources/input.txt")?;
    let buff = BufReader::new(file);

    let map: Vec<Vec<char>> = buff.lines().map(|line| line.unwrap().chars().collect()).collect();

    let mut start: (usize, usize) = (0, 0); // (row, column)
    for (y, row) in map.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if *col == 'S' {
                start = (x, y);
                break;
            }
        }
    }

    let finishing_positions = find_final_places(map, start, 64);
    println!("Finishing positions: {}", finishing_positions);

    return Ok(());
}

fn find_final_places(map: Vec<Vec<char>>, start: (usize, usize), step_count: usize) -> usize {
    let mut starting_places = vec![HashSet::new(); step_count + 1];
    starting_places[0].insert(start);
    let width = map[0].len();
    let height = map.len();

    for nth_step in 0..step_count {
        let starting_places_for_round = starting_places[nth_step].clone();
        for starting_place in starting_places_for_round.iter() {
            for direction in Direction::get_directions() {
                let new_place = direction.displace(starting_place, (height, width));
                if let Some(new_place) = new_place {
                    if map[new_place.0][new_place.1] == '#' {
                        continue;
                    }
                    starting_places[nth_step + 1].insert(new_place);
                }
            }
        }
    }

    // for starting_place in &starting_places {
    //     println!("{:?}", starting_place);
    // }

    starting_places[step_count].len()
}