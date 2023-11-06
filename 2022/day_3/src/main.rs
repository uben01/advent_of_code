use std::{fs::File, io::{BufRead, BufReader}, error::Error, collections::HashSet};

fn letter_to_priority(c: char) -> i32 {
    let char_as_num = c as i32;

    if char_as_num > 96 {
        return char_as_num - 96;
    }

    return char_as_num - 65 + 27;
}


fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("resources/input.txt")?;
    let reader = BufReader::new(file);

    let mut misplaced_items: Vec<i32> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let mut item_set: HashSet<i32> = HashSet::new();
        let mut misplaced_for_line: HashSet<i32> = HashSet::new();

        let length = line.clone().len();
        for (idx, character) in line.chars().enumerate() {
            if idx < length / 2 {
                item_set.insert(letter_to_priority(character));

                continue;
            }

            let priority = letter_to_priority(character);
            if item_set.contains(&priority) {
                misplaced_for_line.insert(priority);
            }
        }
        for item in misplaced_for_line {
            misplaced_items.push(item);
        }
    }
    let misplaced_sum = misplaced_items.into_iter().reduce(|acc, e| acc + e).unwrap();
    println!("{misplaced_sum}");

    return Ok(());
}
