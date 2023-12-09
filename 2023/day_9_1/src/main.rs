use std::{error::Error, fs::File, io::{BufReader, BufRead}};

fn main()  -> Result<(), Box<dyn Error>>  {
    let file = File::open("resources/input.txt")?;
    let buff = BufReader::new(file);

    let mut sum = 0;

    for line in buff.lines() {
        let line = line?;
        let split: Vec<i32> = line
            .split(' ')
            .map(|a| a.parse().unwrap())
            .collect();

        let mut differences: Vec<Vec<i32>> = vec![split];
        while differences.last().unwrap().iter().any(|&x| x != 0) {
            let last = differences.last().unwrap();
            let mut difference: Vec<i32> = vec![];

            for i in 0..last.len() - 1 {
                difference.push(last[i + 1] - last[i]);
            }
            differences.push(difference);
        }

        differences.reverse();
        let mut previous_last = 0;
        for difference in &differences {
            let last = difference.last().unwrap();

            previous_last = last + previous_last;
        }
        sum += previous_last;
        println!("{:?}", differences);
        println!("{previous_last}");
    }

    println!("The sum is: {sum}");
    return Ok(());
}
