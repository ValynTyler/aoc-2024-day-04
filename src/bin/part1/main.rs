use aoc_2024_day4::{grid::Grid, xmas::{BadDataError, XmasGrid}};

fn main() -> Result<(), BadDataError> {
    let grid = Grid(vec![vec![1, 2, 3, 4], vec![1, 3, 2, 4]]);
    println!("{}", grid);

    let input_string = include_str!("../../../input/example.txt");
    let crossword = XmasGrid::try_from(input_string);
    println!("{}", crossword?);

    Ok(())
}
