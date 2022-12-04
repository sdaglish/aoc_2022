use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

type BoxResult<T> = Result<T, Box<dyn Error>>;

fn read_and_process_data(filename: &str) -> BoxResult<Vec<Vec<u32>>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut results: Vec<Vec<u32>> = Vec::new();

    // for line in reader.lines() {
    //     match line? {
    //         s=> {
    //             results.push(s);
    //         }
    //     }
    // }

    for line in reader.lines() {
        match line? {
            l =>  {
                let s: Vec<&str>  = l.split(|c: char| (c == '-') || (c == ',')).collect();
                let mut n: Vec<u32> = Vec::new();
                for S in s {
                    n.push(S.to_string().parse::<u32>().unwrap());
                }
                results.push(n);
                // println!("Split string {:?}", s);
            }
        }
    }

    println!("Data: {:?}", results);
    return Ok(results);
}

fn part1_answer(data: Vec<Vec<u32>>) -> u32 {
    let mut answer = 0;

    for set in data {
        println!("Set1: {:?}" ,set[0]..set[1]);
        println!("Set2: {:?}" ,set[2]..set[3]);
        let mut a_contains_b = true;
        let mut b_contains_a = true;
        for s in set[2]..(set[3]+1) {
            println!("s: {:?}", s);
            if (s < set[0]) || (s > set[1]) {
               a_contains_b = false;
               break;
            }
        }
        for s in set[0]..(set[1]+1) {
            println!("s: {:?}", s);
            if (s < set[2]) || (s > set[3]) {
                b_contains_a = false;
                break;
            }
        }
        if a_contains_b || b_contains_a {
            println!("Set1 contains set 2");
            answer += 1;
        }
    }

    return answer;
}

fn main() {
    match crate::read_and_process_data("data/day4_data.txt") {
        Err(e) => {
            println!("Error {}", e);
            assert_eq!(1, 0);
        }
        Ok(data) => {
            let result = crate::part1_answer(data);
            println!("Day 4, Part 1 answer: {}", result);
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn day4_part1_test() {
        match crate::read_and_process_data("data/day4_test_data.txt") {
            Err(e) => {
                println!("Error {}", e);
                assert_eq!(1, 0);
            }
            Ok(data) => {
                let result = crate::part1_answer(data);
                assert_eq!(result, 2);
            }
        }
    }

    #[test]
    fn day4_part2_test() {
        // match aoc_2022::read_strings_from_file("data/day3_test_data.txt") {
        //     Err(e) => {
        //         println!("Error {}", e);
        //         assert_eq!(1, 0);
        //     }
        //     Ok(strings) => {
        //         let result = crate::part2_answer(strings);
        //         assert_eq!(result, 70);
        //     }
        // }
    }
}
