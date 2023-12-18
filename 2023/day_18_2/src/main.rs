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
            '3' => Ok(Direction::Up),
            '1' => Ok(Direction::Down),
            '2' => Ok(Direction::Left),
            '0' => Ok(Direction::Right),
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
        let (_direction, _distance, color) = (split[0], split[1], split[2]);

        let color = color;
        let mut color = color.trim().to_string();
        color = color.replace('#', "");
        color = color.replace('(', "");
        color = color.replace(')', "");
        let color = color.split_at(5);
        let distance = usize::from_str_radix(color.0, 16)?;

        let direction = &Direction::from_char(color.1.chars().last().unwrap())?;

        walls.push((direction.clone(), distance.clone()));
    }

    let mut borders = walls.clone().iter().map(|x| x.1).sum::<usize>();
    borders /= 2;
    borders += 1;

    let coordinates = calculate_coordinates_from_directions(walls);
    let area = shoelace_formula(coordinates);

    println!("Area: {}", area as usize + borders);

    return Ok(());
}

fn calculate_coordinates_from_directions(walls: Vec<(Direction, usize)>) -> Vec<(isize, isize)> {
    let mut coordinates: Vec<(isize, isize)> = vec![];
    let mut current_coordinate = (0, 0);
    for wall in walls {
        let (x, y) = wall.0.to_coordinates(wall.1);
        current_coordinate = (current_coordinate.0 + x, current_coordinate.1 + y);
        coordinates.push(current_coordinate);
    }

    coordinates
}

fn shoelace_formula(coordinates: Vec<(isize, isize)>) -> isize {
    let mut area = 0;
    let n = coordinates.len();

    for i in 0..n {
        let (x1, y1) = coordinates[i];
        let (x2, y2) = coordinates[(i + 1) % n];
        area += x1 * y2 - x2 * y1;
    }

    area = isize::abs(area) / 2;

    return area;
}