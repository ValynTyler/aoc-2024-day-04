use aoc_2024_day4::xmas_crossword::{BadInputError, XmasCrossword};
use solved_crossword::SolvedCrossword;

mod solved_crossword;

fn main() -> Result<(), BadInputError> {
    let input_string = include_str!("../../../input/example.txt");

    let crossword: XmasCrossword = input_string.try_into()?;
    println!("{}", crossword);

    let solution: SolvedCrossword = crossword.into();
    println!("{}", solution);

    Ok(())
}
