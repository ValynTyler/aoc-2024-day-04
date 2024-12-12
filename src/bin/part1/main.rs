use aoc_2024_day4::xmas::{BadDataError, XmasGrid};
use aoc_2024_day4::vec::Vec2USize as Vec2U;

fn main() -> Result<(), BadDataError> {
    let input_string = include_str!("../../../input/example.txt");
    let crossword = XmasGrid::try_from(input_string)?;
    println!("{}", crossword);

    for i in 0..7 {
        println!("{:?}", crossword.grid().get(Vec2U(i, 0)));
    }

    Ok(())
}
