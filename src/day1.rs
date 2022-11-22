use aoc_2022;

fn main() {
    match aoc_2022::read_one_number_per_line("data/test.txt") {
        Ok(number_vector) => {
            for number in number_vector {
                println!("{}", number);
            }
        }
        Err(e) => {
            println!("Error {}", e);
        }
    }
}