use std::{error::Error, fs::File, io::{BufReader, BufRead}};
use std::collections::HashSet;
use std::str::Split;
use itertools::Itertools;

fn main()  -> Result<(), Box<dyn Error>>  {
    let file = File::open("resources/input.txt")?;
    let buff = BufReader::new(file);

    let mut bricks: Vec<(usize, (usize, usize, usize), (usize, usize, usize))> = vec![];
    for line in buff.lines() {
        let line = line?;

        let mut split = line.split("~");
        let left_bottom = get_coordinates(&mut split);
        let right_top = get_coordinates(&mut split);

        bricks.push((0, left_bottom, right_top));
    }

    let max_x = bricks.iter().map(|(_, _, (x, _, _))| *x).max().unwrap() + 1;
    let max_y = bricks.iter().map(|(_, _, (_, y, _))| *y).max().unwrap() + 1;
    let max_z = bricks.iter().map(|(_, _, (_, _, z))| *z).max().unwrap() + 1;

    let mut map: Vec<Vec<Vec<usize>>> = vec![vec![vec![0; max_z]; max_y]; max_x];

    for (idx, (id, from, to)) in bricks.iter_mut().enumerate() {
        let idx = idx + 1;
        *id = idx;
        for x in from.0..=to.0 {
            for y in from.1..=to.1 {
                for z in from.2..=to.2 {
                    map[x][y][z] = idx;
                }
            }
        }
    }

    loop {
        let mut dropped = false;
        for brick in bricks.iter_mut() {
            if drop_brick(brick, &mut map) {
                dropped = true;
            }
        }

        if !dropped {
            break;
        }
    }

    let mut sum = 0;
    for brick_id in 0..bricks.len() {
        let mut map = map.clone();
        let mut bricks = bricks.clone();
        disintegrate_brick(bricks[brick_id].0, &mut map);

        let mut dropped_ids: HashSet<usize> = HashSet::new();
        loop {
            let mut dropped = false;
            for brick in bricks.iter_mut() {
                if drop_brick(brick, &mut map) {
                    dropped = true;
                    dropped_ids.insert(brick.0);
                }
            }

            if !dropped {
                break;
            }
        }
        sum += dropped_ids.len();

    }

    println!("Result: {}", sum);

    return Ok(());
}

#[allow(dead_code)]
fn print_structure(max_x: usize, max_y: usize, max_z: usize, map: &Vec<Vec<Vec<usize>>>) {
    for z in (0..max_z).rev() {
        for x in 0..max_x {
            for y in 0..max_y {
                print!("{} ", map[x][y][z]);
            }
            println!();
        }
        println!("-----");
    }
    println!("====================");
}

fn get_coordinates(split: &mut Split<&str>) -> (usize, usize, usize) {
    split
      .next()
      .unwrap()
      .split(',')
      .map(|x| x.parse::<usize>().unwrap())
      .collect_tuple()
      .unwrap()
}

fn drop_brick(brick: &mut (usize, (usize, usize, usize), (usize, usize, usize)), map: &mut Vec<Vec<Vec<usize>>>) -> bool {
    if is_supported(brick, map) {
        return false;
    }

    let (id, from, to) = brick;
    let (from_x, from_y, from_z) = from;
    let (to_x, to_y, to_z) = to;

    // drop brick
    *from_z -= 1;
    *to_z -= 1;

    // update map
    for x in *from_x..=*to_x {
        for y in *from_y..=*to_y {
            map[x][y][*from_z] = *id;
            map[x][y][*to_z + 1] = 0;
        }
    }

    return true;
}

fn is_supported(brick: &(usize, (usize, usize, usize), (usize, usize, usize)), map: &Vec<Vec<Vec<usize>>>) -> bool {
    let (_, from, to) = brick;
    let (from_x, from_y, from_z) = from;
    let (to_x, to_y, _) = to;

    if *from_z == 0 {
        return true;
    }
    for x in *from_x..=*to_x {
        for y in *from_y..=*to_y {
            if map[x][y][from_z - 1] != 0 {
                return true;
            }
        }
    }

    return false;
}

fn disintegrate_brick(brick_id: usize, map: &mut Vec<Vec<Vec<usize>>>) {
    let (max_x, max_y, max_z) = (map.len(), map[0].len(), map[0][0].len());
    for x in 0..max_x {
        for y in 0..max_y {
            for z in 0..max_z {
                if map[x][y][z] == brick_id {
                    map[x][y][z] = 0;
                }
            }
        }
    }
}