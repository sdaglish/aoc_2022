use std::fs::File;
use std::io::{BufReader, BufRead};
use std::error::Error;

type BoxResult<T> = Result<T, Box<dyn Error>>;

pub fn read_one_number_per_line(filename: &str) -> BoxResult<Vec<i32>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut numbers: Vec<i32> = Vec::new();

    for line in reader.lines() {
        match line?.parse::<i32>() {
            Ok(l) => {
                numbers.push(l);
            },
            Err(e) => {
                return Err(Box::new(e))
            }
        }
    }

    Ok(numbers)
}
