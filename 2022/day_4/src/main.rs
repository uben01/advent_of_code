use std::{error::Error, fs::File, io::{BufReader, BufRead}};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("resources/input.txt")?;
    let reader = BufReader::new(file);

    let mut counter = 0;
    for line in reader.lines() {
        let line = line?;
        let mut elf_range_iter = line.split(',');

        let left: Vec<i32> = elf_range_iter.next().unwrap().split('-').map(|e| e.parse::<i32>().unwrap()).collect();
        let right: Vec<i32> = elf_range_iter.next().unwrap().split('-').map(|e| e.parse::<i32>().unwrap()).collect();
    
        if left[0] < right[0] {
            if left[1] >= right[1] {
                counter += 1;
            }
        } else if right[0] < left[0] {
            if right[1] >= left[1] {
                counter +=1;
            }
        } else {
            counter += 1;
        }
    }
    
    println!("{counter}");

    return Ok(());
}
