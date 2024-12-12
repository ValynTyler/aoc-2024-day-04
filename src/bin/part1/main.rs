use aoc_2024_day4::grid::GridDirection;
use aoc_2024_day4::vec::{Vec2ISize, Vec2USize};
use aoc_2024_day4::xmas::{BadDataError, XmasGrid, XmasLetter};

fn main() -> Result<(), BadDataError> {
    let input_string = include_str!("../../../input/example.txt");
    let crossword = XmasGrid::try_from(input_string)?;
    println!("{}", crossword);

    use XmasLetter::*;
    use GridDirection::*;
    let xmas = vec![X, M, A, S];
    let directions = vec![
        North,
        South,
        East,
        West,
        NorthEast,
        NorthWest,
        SouthEast,
        SouthWest,
    ];

    let mut sum = 0;
    for dir in directions {
        let pos = Vec2ISize(5, 0);
        let mut flag = true;
        for i in 0..4 {
            let offset = dir.delta() * i as isize;
            if let Ok(coords) = Vec2USize::try_from(pos + offset) {
                let letter = crossword.grid().get(coords).unwrap();
                if letter != xmas[i] { flag = false }
            } else {
                flag = false
            }
        }
        println!("{}", flag);
        sum += flag as u32
    }
    println!("{}", sum);

    Ok(())
}
