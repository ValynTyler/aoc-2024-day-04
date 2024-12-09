use aoc_2024_day4::xmas_crossword::XmasCrossword;

mod solved_crossword;
mod xmas_direction;

fn main() {
    let input_string = include_str!("../../../input/example.txt");
    let crossword: XmasCrossword = input_string.try_into().unwrap();

    println!("{}", crossword);
}
