use std::{error::Error, fs::File, io::{BufReader, BufRead}};
use priority_queue::DoublePriorityQueue;

#[derive(PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn main()  -> Result<(), Box<dyn Error>>  {
    let file = File::open("resources/input.txt")?;
    let buff = BufReader::new(file);

    let mut map: Vec<Vec<u32>> = vec![];
    for line in buff.lines() {
        let line = line?;

        let row: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        map.push(row);
    }

    let mut queue: DoublePriorityQueue<((usize, usize), Option<&Direction>, usize), u32> = DoublePriorityQueue::new();
    queue.push(((0, 0), None, 0), 0);

    let possible_directions = vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right];
    while !queue.is_empty() {
        let (((y, x), direction, direction_count), distance) = queue.pop_min().unwrap();
        if y == map.len() - 1 && x == map[0].len() - 1 {
            println!("Distance: {}", distance);
            break;
        }

        let possible_directions: Vec<&Direction> = possible_directions
            .iter()
            .clone()
            .filter(|a|
                return if direction.is_none() {
                    true
                } else {
                    **a != opposite_direction(&direction.unwrap())
                })
            .filter(|a| {
                if direction_count < 3 {
                    return true;
                }
                return direction.unwrap() != *a;
            })
            .filter(|a| {
                let mut possible = true;
                if y == 0 {
                    possible = possible && **a != Direction::Up;
                }
                if y == map.len() - 1 {
                    possible = possible && **a != Direction::Down;
                }
                if x == 0 {
                    possible = possible && **a != Direction::Left;
                }
                if x == map[0].len() - 1 {
                    possible = possible && **a != Direction::Right;
                }
                return possible;
            })
            .collect();

        for new_direction in possible_directions {
            let (to, distance_to) = calculate_distance_to_point((y, x), &map, new_direction, distance);

            let new_direction_count = if direction.is_some() && direction.unwrap() == new_direction {
                direction_count + 1
            } else {
                1
            };

            let item = (to, Some(new_direction), new_direction_count);
            let prio = queue.get_priority(&item);
            if prio.is_some() && prio.unwrap() < &distance_to {
                continue;
            }

            queue.push(item, distance_to);
        }
    }

    return Ok(());
}

fn calculate_distance_to_point(
    from: (usize, usize),
    map: &Vec<Vec<u32>>,
    direction: &Direction,
    distance: u32,
) -> ((usize, usize), u32) {
    let to = match direction {
        Direction::Up => (from.0 - 1, from.1),
        Direction::Down => (from.0 + 1, from.1),
        Direction::Left => (from.0, from.1 - 1),
        Direction::Right => (from.0, from.1 + 1)
    };
    let distance_to = distance + map[to.0][to.1];

    return (to, distance_to);
}

fn opposite_direction(direction: &Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Down,
        Direction::Down => Direction::Up,
        Direction::Left => Direction::Right,
        Direction::Right => Direction::Left
    }
}