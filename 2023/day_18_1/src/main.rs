use std::{error::Error, fs::File, io::{BufReader, BufRead}};

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(char: char) -> Result<Direction, Box<dyn Error>> {
        match char {
            'U' => Ok(Direction::Up),
            'D' => Ok(Direction::Down),
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(Box::from("Invalid direction"))
        }
    }

    fn to_coordinates(&self, distance: usize) -> (isize, isize) {
        match self {
            Direction::Down => (1 * distance as isize, 0),
            Direction::Up => (-1 * distance as isize, 0),
            Direction::Left => (0, -1 * distance as isize),
            Direction::Right => (0, 1 * distance as isize),
        }
    }
}

fn main()  -> Result<(), Box<dyn Error>>  {
    let file = File::open("resources/input.txt")?;
    let buff = BufReader::new(file);

    let mut walls: Vec<(Direction, usize)> = vec![];
    for line in buff.lines() {
        let line = line?;
        let split: Vec<&str> = line.split(" ").collect();
        let (direction, distance, _color) = (split[0], split[1], split[2]);

        let direction = &Direction::from_char(direction.chars().last().unwrap())?;
        let distance = &distance.parse::<usize>()?;

        walls.push((direction.clone(), distance.clone()));
    }

    let mut start_point = (0, 0);

    let mut map: Vec<Vec<bool>> = vec![];
    map.push(vec![true]);

    for (direction, distance) in walls {
        start_point = extend_map(&mut map, start_point, &direction, &distance);

        let (y, x) = direction.to_coordinates(distance);
        let new_point = ((start_point.0 as isize + y) as usize, (start_point.1 as isize + x) as usize);

        for i in usize::min(start_point.0, new_point.0)..=usize::max(start_point.0, new_point.0) {
            for j in usize::min(start_point.1, new_point.1)..=usize::max(start_point.1, new_point.1) {
                map[i][j] = true;
            }
        }

        start_point = new_point;
    }

    // count trues
    let mut count_border = 0;
    for row in map.iter() {
        for cell in row.iter() {
            if *cell {
                count_border += 1;
            }
        }
    }

    // extend map to every direction
    map.insert(0, vec![false; map[0].len()]);
    map.insert(map.len(), vec![false; map[0].len()]);
    for row in map.iter_mut() {
        row.insert(0, false);
        row.insert(row.len(), false);
    }

    set_outer_to_true(&mut map, (0, 0));

    // count falses
    let mut count_inside = 0;
    for row in map.iter() {
        for cell in row.iter() {
            if !*cell {
                count_inside += 1;
            }
        }
    }

    println!("Count: {}", count_border + count_inside);

    return Ok(());
}

fn extend_map(
    map: &mut Vec<Vec<bool>>,
    from: (usize, usize),
    direction: &Direction,
    distance: &usize
) -> (usize, usize)
{
    match direction {
        Direction::Down => {
            let difference: isize = from.0 as isize + *distance as isize - map.len() as isize;
            for _ in 0..=difference {
                map.push(vec![false; map[0].len()]);
            }

            return from;
        },
        Direction::Right => {
            let difference: isize = from.1 as isize + *distance as isize - map[0].len() as isize;
            for row in map.iter_mut() {
                for _ in 0..=difference {
                    row.push(false);
                }
            }

            return from;
        },
        Direction::Up => {
            let difference: isize = from.0 as isize - *distance as isize;
            if difference < 0 {
                for _ in difference..0 {
                    map.insert(0, vec![false; map[0].len()])
                }

                return ((from.0 as isize - difference) as usize, from.1);
            }
            return from
        },
        Direction::Left => {
            let difference: isize = from.1 as isize - *distance as isize;
            if difference < 0 {
                for row in map.iter_mut() {
                    for _ in difference..0 {
                        row.insert(0, false);
                    }
                }

                return (from.0, (from.1 as isize - difference) as usize);
            }
            return from;
        },
    }
}

fn set_outer_to_true(map: &mut Vec<Vec<bool>>, start: (usize, usize)) {
    let mut queue: Vec<(usize, usize)> = vec![];
    queue.push(start);

    while !queue.is_empty() {
        let start = queue.pop().unwrap();

        map[start.0][start.1] = true;
        for direction in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_point = ((start.0 as isize + direction.0) as usize, (start.1 as isize + direction.1) as usize);
            if new_point.0 < map.len() && new_point.1 < map[0].len() && !map[new_point.0][new_point.1] {
                queue.push(new_point);
            }
        }
    }
}