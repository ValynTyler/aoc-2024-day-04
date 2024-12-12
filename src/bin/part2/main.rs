use aoc_2024_day4::{grid::GridDirection, vec::Vec2ISize, xmas::{BadDataError, XmasGrid, XmasLetter}};

fn main() -> Result<(), BadDataError> {
    let input_string = include_str!("../../../input/crossword.txt");
    let crossword = XmasGrid::try_from(input_string)?;
    println!("{}", crossword);

    let width = crossword.grid().width();
    let height = crossword.grid().height();

    use GridDirection::*;
    use XmasLetter::*;

    let mut sum = 0;
    for i in 0..width {
        for j in 0..height {
            let pos = Vec2ISize(i as isize, j as isize);
            let valid = (crossword.grid().get(pos.try_into().unwrap()) == Some(A)) && ((
                if let Ok(coords) = (pos + NorthWest.delta()).try_into() { crossword.grid().get(coords) == Some(M) } else { false } &&
                if let Ok(coords) = (pos + SouthEast.delta()).try_into() { crossword.grid().get(coords) == Some(S) } else { false }
            ) || (
                if let Ok(coords) = (pos + NorthWest.delta()).try_into() { crossword.grid().get(coords) == Some(S) } else { false } &&
                if let Ok(coords) = (pos + SouthEast.delta()).try_into() { crossword.grid().get(coords) == Some(M) } else { false }
            )) && ((
                if let Ok(coords) = (pos + NorthEast.delta()).try_into() { crossword.grid().get(coords) == Some(M) } else { false } &&
                if let Ok(coords) = (pos + SouthWest.delta()).try_into() { crossword.grid().get(coords) == Some(S) } else { false }
            ) || (
                if let Ok(coords) = (pos + NorthEast.delta()).try_into() { crossword.grid().get(coords) == Some(S) } else { false } &&
                if let Ok(coords) = (pos + SouthWest.delta()).try_into() { crossword.grid().get(coords) == Some(M) } else { false }
            ));

            sum += valid as u32
        }
    }
    println!("{}", sum);

    Ok(())
}
