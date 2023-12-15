use std::{error::Error, fmt, fs::File, io::{BufReader, BufRead}};

#[derive(Clone)]
struct Pipe {
    char: char,
    changed: bool
}

impl fmt::Display for Pipe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "'{}'", self.char)
    }
}

impl PartialEq for Pipe {
    fn eq(&self, other: &Self) -> bool {
        return self.char == other.char;
    }
}

fn main()  -> Result<(), Box<dyn Error>>  {
    let file = File::open("resources/input.txt")?;
    let buff = BufReader::new(file);

    let mut sum = 0;
    for line in buff.lines() {
        let line = line?;

        let mut split = line.split(' ');
        let pipes: Vec<Pipe> = split
            .next()
            .unwrap()
            .chars()
            .map(|c| Pipe {char: c, changed: false})
            .collect();
        let broken_pipes: Vec<usize> = split
            .next()
            .unwrap()
            .split(',')
            .map(|c| c.parse().unwrap())
            .collect();

        let broken_count = broken_pipes.clone().iter().sum();

        let mut found: Vec<Vec<Pipe>> = vec![];
        let count_this_row = try_out_with(pipes, broken_pipes, &mut found, 0, broken_count);
        sum += count_this_row;

        println!("{count_this_row}");
    }
    println!("{sum}");

    return Ok(());
}

fn try_out_with(pipes: Vec<Pipe>, broken_pipes: Vec<usize>, found: &mut Vec<Vec<Pipe>>, start: usize, original_broken_sum: usize) -> usize {
    if broken_pipes.len() == 0 {
        let mut pipes = pipes.clone();

        for pipe_id in 0..pipes.len() {
            let pipe = pipes.get_mut(pipe_id).unwrap();
            if pipe.char == '?' {
                *pipe = Pipe {char: '.', changed: true};
            }
        }

        if found.contains(&pipes) {
            return 0;
        }

        let broken_count = pipes.clone().iter().filter(|x| x.char == '#').count();

        if broken_count != original_broken_sum {
            return 0;
        }

        // print!("[");
        // for pipe in &pipes {
        //     print!("{pipe},");
        // }
        // println!("]");
        found.push(pipes);
        return 1;
    }

    let mut count = 0;
    let broken_len = *broken_pipes.get(0).unwrap();

    for start in start..pipes.len() {
        if will_fit(&pipes, start, broken_len) {
            let mut pipes = pipes.clone();
            set_from_n(&mut pipes, start, broken_len);

            let mut brokens = broken_pipes.clone();
            brokens.remove(0);

            count += try_out_with(pipes, brokens, found, start + broken_len, original_broken_sum);
        }
    }

    return count;
}

fn will_fit(pipes: &Vec<Pipe>, start: usize, len: usize) -> bool {
    let split = pipes.split_at(start);
    let previous = split.0;
    let remaining = split.1;

    if remaining.len() < len {
        return false;
    }

    if previous.len() != 0 {
        if previous.last().unwrap().char == '#' {
            return false;
        }
    }

    if remaining.len() > len {
        if remaining.get(len).unwrap().char == '#'{
            return false;
        }
    }

    return remaining.iter().take(len).all(|c| c.char != '.' && !c.changed);
}

fn set_from_n(pipes: &mut Vec<Pipe>, from: usize, n: usize) {
    for i in from..from + n {
        let char = pipes.get_mut(i).unwrap();
        *char =  Pipe { char: '#', changed: true };
    }

    if from as i32 - 1 >= 0 {
        let char = pipes.get_mut((from as i32 - 1) as usize).unwrap();
        *char = Pipe {char: '.', changed: true };
    }

    if from + n < pipes.len() {
        let char = pipes.get_mut(from + n).unwrap();
        *char = Pipe {char: '.', changed: true};
    }
}