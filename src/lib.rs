use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

type BoxResult<T> = Result<T, Box<dyn Error>>;

/* TODO: Currently not used any more. Can probably be removed */
#[derive(Debug)]
pub enum MoveT {
    A,
    B,
    C,
    X,
    Y,
    Z,
}

pub fn read_one_number_per_line(filename: &str) -> BoxResult<Vec<i32>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut numbers: Vec<i32> = Vec::new();

    for line in reader.lines() {
        numbers.push(line?.parse::<i32>()?);
    }

    Ok(numbers)
}

pub fn read_on_number_per_line_with_gaps_to_negative_one(filename: &str) -> BoxResult<Vec<i32>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut numbers: Vec<i32> = Vec::new();

    for line in reader.lines() {
        match line?.parse::<i32>() {
            Err(_e) => numbers.push(-1),
            Ok(n) => numbers.push(n),
        }
    }

    Ok(numbers)
}

pub fn read_rock_paper_scissors_data_from_file(filename: &str) -> BoxResult<Vec<Vec<char>>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut results: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        match line? {
            s => {
                let chars: Vec<char> = s.chars().collect();
                let mut l: Vec<char> = Vec::new();
                l.push(chars[0]);
                l.push(chars[2]);

                results.push(l);
            }
        }
    }
    // println!("Result = {:?}", results);

    Ok(results)
}

/*
* A = rock = 1
* B = paper = 2
* C = scissors = 3
 */

pub fn read_strings_from_file(filename: &str) -> BoxResult<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut results: Vec<String> = Vec::new();

    for line in reader.lines() {
        match line? {
            s => {
                results.push(s);
            }
        }
    }

    return Ok(results);
}
