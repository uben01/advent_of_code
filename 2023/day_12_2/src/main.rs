use std::{error::Error, fs::File, io::{BufReader, BufRead}};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("resources/input.txt")?;

    let buf = BufReader::new(file);

    let mut sum = 0;
    for line in buf.lines() {
        let line = line?;
        let mut split = line.split(' ');
        let pipes: Vec<char> = split
            .next()
            .unwrap()
            .chars()
            .collect();
        let broken_pipes: Vec<usize> = split
            .next()
            .unwrap()
            .split(',')
            .map(|c| c.parse().unwrap())
            .collect();

        let (pipes, broken_pipes) = multiply_pipes(&mut pipes.clone(), &mut broken_pipes.clone());

        let mut cache: HashMap<String, usize> = HashMap::new();

        let row_value = place_pipes(&pipes, &broken_pipes, &mut cache);
        // println!("{row_value}");
        sum += row_value;
    }

    println!("{sum}");

    return Ok(());
}

fn multiply_pipes(pipes: &mut Vec<char>, broken_pipes: &mut Vec<usize>) -> (Vec<char>, Vec<usize>) {
    let mut new_pipes: Vec<char> = vec![];
    for _ in 0..5 {
        new_pipes.append(pipes.clone().as_mut());
        new_pipes.push('?');
    }
    new_pipes.pop();
    let broken_pipes = broken_pipes.repeat(5);

    return (new_pipes, broken_pipes);
}

fn place_pipes(pipes: &Vec<char>, broken_pipes: &Vec<usize>, cache: &mut HashMap<String, usize>) -> usize {
    if broken_pipes.len() == 0 {
        return if pipes.contains(&'#') { 0 } else { 1 };
    }

    let hash = hash(pipes, broken_pipes);
    if cache.contains_key(&hash) {
        return *cache.get(&hash).unwrap();
    }

    let mut sum = 0;

    let mut new_broken_pipes = broken_pipes.clone();
    let current_broken = new_broken_pipes.remove(0);

    let mut found_min_starter = false;
    for start in 0..pipes.len() {
        if pipes[start] == '#' {
            found_min_starter = true;
        }

        let remaining_len = pipes.len() - start;
        if remaining_len < current_broken {
            cache.insert(hash, sum);
            return sum;
        }

        if will_fit(pipes, start, current_broken) {
            let mut next_start = start + current_broken;

            if pipes.len() >= next_start + 1 {
                next_start += 1;
            }

            let pipes = pipes.clone().split_off(next_start);
            sum += place_pipes(&pipes, &mut new_broken_pipes, cache);
        }

        if found_min_starter {
            break;
        }
    }

    cache.insert(hash, sum);
    return sum;
}

fn will_fit(pipes: &Vec<char>, start: usize, len: usize) -> bool {
    let split = pipes.split_at(start);
    let previous = split.0;
    let remaining = split.1;

    if remaining.len() < len {
        return false;
    }

    if previous.len() != 0 {
        if previous.last().unwrap() == &'#' {
            return false;
        }
    }

    if remaining.len() > len {
        if remaining.get(len).unwrap() == &'#'{
            return false;
        }
    }

    return remaining.iter().take(len).all(|c| c != &'.');
}

fn hash(pipes: &Vec<char>, broken_pipes: &Vec<usize>) -> String {
    pipes.iter().collect::<String>() + broken_pipes.iter().map(|a| a.to_string()).collect::<String>().as_str()
}