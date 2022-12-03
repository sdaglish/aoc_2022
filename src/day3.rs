fn part1_answer(rucksack_strings: Vec<String>) -> u32 {
    let mut answer:u32 = 0;
    let mut rucksack_numbers: Vec<Vec<u32>> = Vec::new();

    for mut rucksack in rucksack_strings {

        let mut items_checked:[u32; ((26 * 2) + 6)] = [0; ((26 * 2) + 6)];
        let rucksack_size = rucksack.len();
        let second = rucksack.split_off(rucksack_size / 2);

        for compartment_1_item in rucksack.chars() {
            for compartement_2_item in second.chars() {
                if compartment_1_item == compartement_2_item {
                    let index = compartment_1_item as usize;
                    items_checked[index - 65] += 1;
                }
            }
        }

        for (i, item) in items_checked.iter().enumerate() {
            if *item > 0 {
                if i < 26 {
                    // Uppercase
                    answer += ((i as u32) + 27);
                }
                else {
                    // Lowercase
                    answer += ((i as u32) - 26 - 5);
                }
            }
        }
    }

    return answer;
}

fn main() {
    match aoc_2022::read_strings_from_file("data/day3_data.txt") {
        Err(e) => {
            println!("Error {}", e);
        }
        Ok(rucksack_strings) => {
            let result = crate::part1_answer(rucksack_strings);
            println!("Day 3 part 1 answer: {}", result);
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn day3_part1_test() {
        match aoc_2022::read_strings_from_file("data/day3_test_data.txt") {
            Err(e) => {
                println!("Error {}", e);
                assert_eq!(1, 0);
            }
            Ok(rucksack_strings) => {
                println!("{:?}", rucksack_strings);
                let result = crate::part1_answer(rucksack_strings);
                assert_eq!(result, 157);
                // println!("Day 2 part 1 answer: {}", result);
            }
        }
    }

    // #[test]
    // fn day2_part2_test() {
    //     match aoc_2022::read_rock_paper_scissors_data_from_file("data/day2_test_data.txt") {
    //         Err(e) => {
    //             println!("Error {}", e);
    //             assert_eq!(1, 0);
    //         }
    //         Ok(number_vector) => {
    //             let result = crate::part2_answer(&number_vector);
    //             assert_eq!(result, 12);
    //             println!("Day 2 part 2 answer: {}", result);
    //         }
    //     }
    // }
}
