
fn part1_answer(initial_stack: Vec<Vec<char>>, move_list: Vec<[u32; 3]>) -> String {
   let mut answer: String = "".to_string();

    let mut stack = initial_stack;

    for moves in move_list {
        println!("Move: {:?}", moves);
        for i in 0..moves[0] {
            let item = stack[(moves[1] - 1) as usize].pop().unwrap();
            stack[(moves[2] - 1) as usize].push(item);
        }
        println!("Stack: {:?}", stack);
    }

    for s in stack {
        answer.push(*s.last().unwrap());
    }

    return answer;
}

fn part2_answer(initial_stack: Vec<Vec<char>>, move_list: Vec<[u32; 3]>) -> String {
    let mut answer: String = "".to_string();

    let mut stack = initial_stack;

    for moves in move_list {
        println!("Move: {:?}", moves);
        let mut stack_slice: Vec<char> = Vec::new();
        for i in 0..moves[0] {
            let item = stack[(moves[1] - 1) as usize].pop().unwrap();
            stack_slice.push(item);
        }
        for i in 0..moves[0] {
            let item = stack_slice.pop().unwrap();
            stack[(moves[2] - 1) as usize].push(item);
        }
        println!("Stack: {:?}", stack);
    }

    for s in stack {
        answer.push(*s.last().unwrap());
    }

    return answer;
}

fn main() {
    match aoc_2022::read_day_5_data_from_file("data/day5_data.txt") {
        Err(e) => {
            println!("Error {}", e);
            assert_eq!(1, 0);
        }
        Ok((stack, moves)) => {
            let result = crate::part1_answer(stack, moves);
            println!("Day 5 Part 1 answer: {}", result);
        }
    }

    match aoc_2022::read_day_5_data_from_file("data/day5_data.txt") {
        Err(e) => {
            println!("Error {}", e);
            assert_eq!(1, 0);
        }
        Ok((stack, moves)) => {
            let result = crate::part2_answer(stack, moves);
            println!("Day 5 Part 2 answer: {}", result);
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn day5_part1_test() {
        match aoc_2022::read_day_5_data_from_file("data/day5_test_data.txt") {
            Err(e) => {
                println!("Error {}", e);
                assert_eq!(1, 0);
            }
            Ok((stack, moves)) => {
                let result = crate::part1_answer(stack, moves);
                assert_eq!(result, "CMZ");
            }
        }
    }

    #[test]
    fn day5_part2_test() {
        match aoc_2022::read_day_5_data_from_file("data/day5_test_data.txt") {
            Err(e) => {
                println!("Error {}", e);
                assert_eq!(1, 0);
            }
            Ok((stack, moves)) => {
                let result = crate::part2_answer(stack, moves);
                assert_eq!(result, "MCD");
            }
        }
    }
}