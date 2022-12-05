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

fn get_current_stack_from_position(position: u32) -> u32 {
    return (position / 4) + 1;
}

pub fn read_day_5_data_from_file(filename: &str) -> BoxResult<(Vec<Vec<char>>, Vec<[u32; 3]>)> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut initial_stack: Vec<Vec<char>> = Vec::new();
    let mut moves: Vec<[u32; 3]> = Vec::new();
    let mut working_on_stack = true;


    println!("Stack: {:?}", initial_stack);


    for line in reader.lines() {
        match line? {
            line => {
                let characters: Vec<char> = line.chars().collect();
                println!("words: {:?}", line);
                if working_on_stack {
                    for i in 0..characters.len() {
                        print!("{}", characters[i]);
                        if characters[i] == '[' {
                            let current_stack = get_current_stack_from_position(i as u32);
                            /* Making sure we have the correct amount of stacks */
                            while ((initial_stack.len() as u32) < current_stack) {
                                let new_stack: Vec<char> = Vec::new();
                                initial_stack.push(new_stack);
                            }
                            initial_stack[(current_stack - 1) as usize].push(characters[i + 1]);
                        } else if characters[1] == '1' {
                            /* Working on moves */
                            working_on_stack = false;
                            break;
                        }
                    }
                }
                else {
                    /* Working on moves */
                    let mut move_array: [u32; 3] = [0; 3];
                    let mut move_array_index = 0;
                    for word in line.split(' ') {
                        match word.to_string().parse::<u32>() {
                            Ok(number) => {
                                move_array[move_array_index] = number;
                                move_array_index += 1;
                            },
                            Err(_) => {}
                        }
                    }
                    if move_array[0] != 0 {
                        moves.push(move_array);
                    }
                }
                println!("");
            }
        }
    }

    let mut initial_stack_2: Vec<Vec<char>> = Vec::new();
    for mut stack in initial_stack {
        let mut temp_stack: Vec<char> = Vec::new();
        while !stack.is_empty() {
            temp_stack.push(stack.pop().unwrap());
        }
        initial_stack_2.push(temp_stack);
    }

    // moves = moves.
    println!("Stack: {:?}", initial_stack_2);
    println!("Moves: {:?}", moves);

    return Ok((initial_stack_2, moves));
}
