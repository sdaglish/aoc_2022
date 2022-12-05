const LOST_SCORE: i32 = 0;
const DRAW_SCORE: i32 = 3;
const WIN_SCORE: i32 = 6;

const A_SCORE: i32 = 1;
const B_SCORE: i32 = 2;
const C_SCORE: i32 = 3;

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

/*
* X = lost == 0
* Y = draw  == 3
* Z = win == 6

* A = rock = 1
* B = paper = 2
* C = scissors = 3

* lost == 0
* draw == 3
* win == 6

*/
fn part2_answer(moves: &Vec<Vec<char>>) -> i32 {
    let mut result = 0;

    for current_move in moves {
        result += match current_move[0] {
            'A' => {
                match current_move[1] {
                    'X' => {
                        /* lose -> Rock(A) > Scissors(C) */
                        LOST_SCORE + C_SCORE
                    }
                    'Y' => {
                        /* draw -> Rock(A) == Rock(A) */
                        DRAW_SCORE + A_SCORE
                    }
                    'Z' => {
                        /* win -> Rock(A) < Paper(B) */
                        WIN_SCORE + B_SCORE
                    }
                    _ => 0,
                }
            }
            'B' => {
                match current_move[1] {
                    'X' => {
                        /* lose -> paper(B) > rock(A) */
                        LOST_SCORE + A_SCORE
                    }
                    'Y' => {
                        /* draw -> paper(B) == paper(B) */
                        DRAW_SCORE + B_SCORE
                    }
                    'Z' => {
                        /* win -> paper(B) < scissors(C) */
                        WIN_SCORE + C_SCORE
                    }
                    _ => 0,
                }
            }
            'C' => {
                match current_move[1] {
                    'X' => {
                        /* lose -> scissors(C) > paper(B) */
                        LOST_SCORE + B_SCORE
                    }
                    'Y' => {
                        /* draw -> scissors(C) == scissors(C) */
                        DRAW_SCORE + C_SCORE
                    }
                    'Z' => {
                        /* win -> scissors(C) < rock(C) */
                        WIN_SCORE + A_SCORE
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
            println!("Day 2 part 2 answer: {}", part2_answer(&number_vector));
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
                println!("Day 2 part 1 answer: {}", result);
            }
        }
    }

    #[test]
    fn day2_part2_test() {
        match aoc_2022::read_rock_paper_scissors_data_from_file("data/day2_test_data.txt") {
            Err(e) => {
                println!("Error {}", e);
                assert_eq!(1, 0);
            }
            Ok(number_vector) => {
                let result = crate::part2_answer(&number_vector);
                assert_eq!(result, 12);
                println!("Day 2 part 2 answer: {}", result);
            }
        }
    }
}
