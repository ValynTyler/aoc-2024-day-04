use aoc_2024_day4::{grid::Grid, xmas_crossword::BadInputError};

mod solved_crossword;

fn main() -> Result<(), BadInputError> {
    let grid = Grid(vec![vec![1, 2, 3, 4], vec![1, 3, 2, 4]]);
    println!("{}", grid);

    Ok(())
}
