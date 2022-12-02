use aoc_2022::*;

/* Sum up the total calories for each of the elves, push to vector, and arrange by highest */
fn calculate_total_calories_each(numbers: &Vec<i32>) -> Vec<i32> {
    let mut total_calories: Vec<i32> = Vec::new();
    let mut total = 0;

    for number in numbers {
        if *number == -1 {
            total_calories.push(total);
            total = 0;
        } else {
            total += number;
        }
    }
    total_calories.push(total);

    total_calories.sort_by(|a, b| b.cmp(a));

    total_calories
}

fn part1_answer(numbers: &Vec<i32>) -> i32 {
    let total_calories = calculate_total_calories_each(numbers);
    total_calories[0]
}

fn part2_answer(numbers: &Vec<i32>) -> i32 {
    let total_calories = calculate_total_calories_each(numbers);
    total_calories[0] + total_calories[1] + total_calories[2]
}

fn main() {
    match read_on_number_per_line_with_gaps_to_negative_one("data/day1_data.txt") {
        Err(e) => {
            println!("Error {}", e);
        }
        Ok(number_vector) => {
            println!("Day 1 part 1 answer: {}", part1_answer(&number_vector));
            println!("Day 1 part 2 answer: {}", part2_answer(&number_vector));
        }
    }
}

#[cfg(test)]
mod test {
    const NUMBER_VECTOR: [i32; 14] = [
        1000, 2000, 3000, -1, 4000, -1, 5000, 6000, -1, 7000, 8000, 9000, -1, 10000,
    ];

    #[test]
    fn day1_part1_test() {
        let result = crate::part1_answer(&NUMBER_VECTOR.to_vec());
        assert_eq!(result, 24000);
    }

    #[test]
    fn day1_part2_test() {
        let result = crate::part2_answer(&NUMBER_VECTOR.to_vec());
        assert_eq!(result, 45000);
    }
}
