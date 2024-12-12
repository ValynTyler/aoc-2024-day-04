use aoc_2024_day4::grid::GridDirection;
use aoc_2024_day4::xmas::{BadDataError, XmasGrid};

fn main() -> Result<(), BadDataError> {
    let input_string = include_str!("../../../input/example.txt");
    let crossword = XmasGrid::try_from(input_string)?;
    println!("{}", crossword);

    let pos = GridDirection::East.delta() * 1;
    println!("{:?} {:?}", crossword.grid().get(pos.try_into().unwrap()), pos);

    Ok(())
}
