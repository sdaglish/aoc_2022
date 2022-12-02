
/*
* X = rock == 1
* Y = paper  == 2
* Z = scissors == 3

* A = rock
* B = paper
* C = scissors

* lost == 0
* draw == 3
* win == 6

*/
fn part1_answer(moves: &Vec<Vec<char>>) -> i32 {
    let mut result = 0;

    const LOST_SCORE: i32 = 0;
    const DRAW_SCORE: i32 = 3;
    const WIN_SCORE: i32 = 6;

    for current_move in moves {
        result += match current_move[0] {
            'A' => {
                match current_move[1] {
                    'X' => {
                        /* Draw */
                        DRAW_SCORE + 1
                    }
                    'Y' => {
                        /* Win */
                        WIN_SCORE + 2
                    }
                    'Z' => {
                        /* Lost */
                        LOST_SCORE + 3
                    }
                    _ => 0,
                }
            }
            'B' => {
                match current_move[1] {
                    'X' => {
                        /* Lost */
                        LOST_SCORE + 1
                    }
                    'Y' => {
                        /* Draw */
                        DRAW_SCORE + 2
                    }
                    'Z' => {
                        /* Win */
                        WIN_SCORE + 3
                    }
                    _ => 0,
                }
            }
            'C' => {
                match current_move[1] {
                    'X' => {
                        /* Win */
                        WIN_SCORE + 1
                    }
                    'Y' => {
                        /* Lost */
                        LOST_SCORE + 2
                    }
                    'Z' => {
                        /* Draw */
                        DRAW_SCORE + 3
                    }
                    _ => 0,
                }
            }
            _ => 0,
        }
    }

    return result;
}

fn main() {
    match aoc_2022::read_rock_paper_scissors_data_from_file("data/day2_data.txt") {
        Err(e) => {
            println!("Error {}", e);
        }
        Ok(number_vector) => {
            println!("Day 2 part 1 answer: {}", part1_answer(&number_vector));
            // println!("Day 1 part 2 answer: {}", part2_answer(&number_vector));
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn day2_part1_test() {
        match aoc_2022::read_rock_paper_scissors_data_from_file("data/day2_test_data.txt") {
            Err(e) => {
                println!("Error {}", e);
                assert_eq!(1, 0);
            }
            Ok(number_vector) => {
                let result = crate::part1_answer(&number_vector);
                assert_eq!(result, 15);
                // println!("Day 1 part 2 answer: {}", part2_answer(&number_vector));
            }
        }
        // let result = crate::part1_answer(&NUMBER_VECTOR.to_vec());
        // assert_eq!(result, 24000);
    }
}
