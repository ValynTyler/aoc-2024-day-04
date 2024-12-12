use aoc_2024_day4::grid::GridDirection;
use aoc_2024_day4::vec::Vec2USize;
use aoc_2024_day4::xmas::{BadDataError, XmasGrid};

fn main() -> Result<(), BadDataError> {
    let input_string = include_str!("../../../input/example.txt");
    let crossword = XmasGrid::try_from(input_string)?;
    println!("{}", crossword);

    let pos = Vec2USize(4, 0);
    for i in 0..4 {
        let offset = GridDirection::SouthEast.delta() * i;
        let offset = Vec2USize::try_from(offset).unwrap();
        let letter = crossword.grid().get(pos + offset);
        println!("{:?} {:?}", letter, pos + offset);
    }

    Ok(())
}
