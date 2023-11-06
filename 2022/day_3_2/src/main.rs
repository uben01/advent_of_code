use std::{fs::File, io::{BufRead, BufReader}, error::Error, collections::{HashSet, HashMap}};

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
    
    let mut sub_id = 0;
    let mut ids: Vec<i32> = Vec::new();
    let mut chunk_map: HashMap<i32, i8> = HashMap::new();
    for line in reader.lines() {
        let mut line_ids: HashSet<i32> = HashSet::new(); 
        for character in line?.chars() {
            line_ids.insert(letter_to_priority(character));
        }
        
        for possible_id in line_ids {
            let item = chunk_map.entry(possible_id).or_insert(0);
            *item += 1;
        }

        sub_id += 1;

        if sub_id == 3 {
            sub_id = 0;
            for (id, occurrences) in &chunk_map {
                if *occurrences != 3 {
                    continue;
                }
                ids.push(*id);
                chunk_map.clear();
                break;
            }
        }
    }
  
    let id_priority_sum = ids.into_iter().reduce(|acc, e| acc + e).unwrap();
    println!("{id_priority_sum}");

    return Ok(());
}
