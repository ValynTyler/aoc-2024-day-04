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

    let width = crossword.grid().0[0].len();
    let height = crossword.grid().0.len();

    let mut sum = 0;
    for i in 0..width {
        for j in 0..height {
            for dir in &directions {
                let pos = Vec2ISize(i as isize, j as isize);
                let mut flag = true;
                for i in 0..4 {
                    let offset = dir.delta() * i as isize;
                    match Vec2USize::try_from(pos + offset) {
                        Err(_) => flag = false,
                        Ok(coords) =>{
                            match crossword.grid().get(coords) {
                                Some(letter) => if letter != xmas[i] { flag = false },
                                None => flag = false,
                            }
                        }
                    }
                }
                if flag { println!("{:?} {}", pos, dir) }
                sum += flag as u32
            }
        }
    }
    println!("{}", sum);

    Ok(())
}
