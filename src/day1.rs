use aoc_2022::*;

// fn calculate_windows_incrementation(numbers: &Vec<i32>, len: usize) -> i32 {
//     let mut inc = 0;
//     if len > 1 {
//         for slice in numbers.windows(len) {
//             if slice[len - 1] > slice[0] {
//                 inc += 1;
//             }
//         }
//     }
//     inc
// }

fn part1_answer(numbers: &Vec<i32>) -> i32 {
    let mut total_calories: Vec<i32> = Vec::new();
    let mut total = 0;

    for number in numbers {
        if *number == -1 {
            total_calories.push(total);
            total = 0;
        }
        else {
            total += number;
        }
    }
    total_calories.push(total);

    let mut highest = total_calories[0];
    for calories in total_calories {
        if calories > highest {
            highest = calories;
        }
    }

    return highest;
}

// fn part2_answer(numbers: &Vec<i32>) -> i32 {
//     calculate_windows_incrementation(numbers, 4)
// }

fn main() {
    match read_on_number_per_line_with_gaps_to_negative_one("data/day1_data.txt") {
        Err(e) => {
            println!("Error {}", e);
        }
        Ok(number_vector) => {
            println!("Day 1 part 1 answer: {}", part1_answer(&number_vector));
            // println!("Day 1 part 2 answer: {}", part2_answer(&number_vector));
        }
    }
}

#[cfg(test)]
mod test {
    const NUMBER_VECTOR: [i32; 14] = [
        1000,
        2000,
        3000,
        -1,
        4000,
        -1,
        5000,
        6000,
        -1,
        7000,
        8000,
        9000,
        -1,
        10000];

    #[test]
    fn day1_part1_test() {
        let result = crate::part1_answer(&NUMBER_VECTOR.to_vec());
        assert_eq!(result, 24000);
    }

    // #[test]
    // fn day1_part2_test() {
    //     let result = crate::part2_answer(&NUMBER_VECTOR.to_vec());
    //     assert_eq!(result, 5);
    // }
}
