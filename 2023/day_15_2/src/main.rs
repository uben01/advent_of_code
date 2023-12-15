use std::{error::Error, fs::File, io::{BufReader, BufRead}};
use indexmap::{indexmap, IndexMap};

fn main()  -> Result<(), Box<dyn Error>>  {
    let file = File::open("resources/input.txt")?;
    let buff = BufReader::new(file);

    let mut boxes: Vec<IndexMap<String, usize>> = vec![indexmap! {}; 256];

    for line in buff.lines() {
        let line = line?;

        let split = line.split(",");
        for block in split {
            if block.contains('=') {
                let mut split = block.split("=");

                let left = split.next().unwrap().to_string();
                let right: usize = split.next().unwrap().parse().unwrap();
                let box_id = hash(&left);

                boxes.get_mut(box_id).unwrap().insert(left, right);
            } else {
                let block = block.replace('-', "");
                let box_id = hash(&block);

                boxes.get_mut(box_id).unwrap().shift_remove(&block);
            }
        }
    }

    let mut sum = 0;
    for (box_id, map) in boxes.iter().enumerate() {
        for (slot_id, (_, focal_length)) in map.iter().enumerate() {
           sum += (box_id + 1) * (slot_id + 1) * focal_length;
        }
    }

    println!("Sum: {}", sum);

    return Ok(());
}

fn hash(block: &String) -> usize {
    let mut value = 0;
    for char in block.chars() {
        value += char as usize;
        value *= 17;
        value = value % 256;
    }

    value
}
