use std::{fs::File, error::Error, io::{BufRead, BufReader}};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./resources/input.txt")?;
    let reader = BufReader::new(file);

    let mut max_arr = [0; 3];
    let mut current = 0;
    for line in reader.lines() {
        let value = line?.to_string();
        let value = value.trim();
        if value.len() == 0 {
            let min_idx = find_min_idx(max_arr);
            if current > max_arr[min_idx] {
               max_arr[min_idx] = current; 
            }
            current = 0;
            continue;
        }
        
        current += value.parse::<i32>()?;
    }
    println!("{:}", max_arr[0] + max_arr[1] + max_arr[2]);

    return Ok(());
}

fn find_min_idx(arr: [i32; 3]) -> usize {
    let mut min_idx = 0;

    for (i, element) in arr.iter().skip(1).enumerate() {
        if element < &arr[min_idx] {
            min_idx = i+1;
        }
    } 

    return min_idx;
}
