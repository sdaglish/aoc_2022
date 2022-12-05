fn part1_answer(rucksack_strings: Vec<String>) -> u32 {
    let mut answer: u32 = 0;

    for mut rucksack in rucksack_strings {
        let mut items_checked: [u32; ((26 * 2) + 6)] = [0; ((26 * 2) + 6)];
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
                    answer += (i as u32) + 27;
                } else {
                    // Lowercase
                    answer += (i as u32) - 26 - 5;
                }
            }
        }
    }

    return answer;
}

fn part2_answer(rucksacks: Vec<String>) -> u32 {
    let mut answer = 0;

    for group in rucksacks.chunks(3) {
        let mut items_checked_0: [u32; ((26 * 2) + 6)] = [0; ((26 * 2) + 6)];
        let mut items_checked_1: [u32; ((26 * 2) + 6)] = [0; ((26 * 2) + 6)];
        let mut items_checked_2: [u32; ((26 * 2) + 6)] = [0; ((26 * 2) + 6)];

        for item in group[0].chars() {
            items_checked_0[item as usize - 65] = 1;
        }
        for item in group[1].chars() {
            items_checked_1[item as usize - 65] = 1;
        }
        for item in group[2].chars() {
            items_checked_2[item as usize - 65] = 1;
        }

        for i in 0..((26 * 2) + 6) {
            if (items_checked_0[i] > 0) && (items_checked_1[i] > 0) && (items_checked_2[i] > 0) {
                if i < 26 {
                    // Uppercase
                    answer += (i as u32) + 27;
                } else {
                    // Lowercase
                    answer += (i as u32) - 26 - 5;
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
    match aoc_2022::read_strings_from_file("data/day3_data.txt") {
        Err(e) => {
            println!("Error {}", e);
        }
        Ok(rucksack_strings) => {
            let result = crate::part2_answer(rucksack_strings);
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
                let result = crate::part1_answer(rucksack_strings);
                assert_eq!(result, 157);
            }
        }
    }

    #[test]
    fn day3_part2_test() {
        match aoc_2022::read_strings_from_file("data/day3_test_data.txt") {
            Err(e) => {
                println!("Error {}", e);
                assert_eq!(1, 0);
            }
            Ok(strings) => {
                let result = crate::part2_answer(strings);
                assert_eq!(result, 70);
            }
        }
    }
}
