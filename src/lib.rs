use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

type BoxResult<T> = Result<T, Box<dyn Error>>;

pub fn read_one_number_per_line(filename: &str) -> BoxResult<Vec<i32>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut numbers: Vec<i32> = Vec::new();

    for line in reader.lines() {
        numbers.push(line?.parse::<i32>()?);
    }

    Ok(numbers)
}
