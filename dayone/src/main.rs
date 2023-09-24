use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Error};

fn main() -> Result<(), Error> {
    let data = get_data("dayone/assets/input.txt")?;

    let part_one = data
        .windows(2)
        .filter(|&window| &window[0] < &window[1])
        .count();

    let part_two = data
        .windows(3)
        .map(|window| window.iter().sum())
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|&window| &window[0] < &window[1])
        .count();
        
    println!(
        "Number of times a depth measurement increased from a previous measurement: {}",
        part_one
    );

    println!(
        "Number of times a depth measurement increased from a previous measurement: {}",
        part_two
    );

    Ok(())
}

fn get_data(file_name: &str) -> Result<Vec<i32>, Error> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    let mut data = Vec::new();

    for line in reader.lines() {
        let parsed_number = line?.parse::<i32>().unwrap();
        data.push(parsed_number);
    }

    Ok(data)
}
