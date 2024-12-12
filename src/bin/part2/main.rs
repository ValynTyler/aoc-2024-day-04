use aoc_2024_day4::{grid::GridDirection, vec::{Vec2ISize, Vec2USize}, xmas::{BadDataError, XmasGrid, XmasLetter}};

fn main() -> Result<(), BadDataError> {
    let input_string = include_str!("../../../input/example.txt");
    let crossword = XmasGrid::try_from(input_string)?;
    println!("{}", crossword);

    let _width = crossword.grid().width();
    let _height = crossword.grid().height();

    use GridDirection::*;
    use XmasLetter::*;

    let pos = Vec2ISize(2, 1);
    let valid = (crossword.grid().get(pos.try_into().unwrap()) == Some(A)) && ((
        crossword.grid().get((pos + NorthWest.delta()).try_into().unwrap()) == Some(M) &&
        crossword.grid().get((pos + SouthEast.delta()).try_into().unwrap()) == Some(S)
    ) || (
        crossword.grid().get((pos + NorthWest.delta()).try_into().unwrap()) == Some(S) &&
        crossword.grid().get((pos + SouthEast.delta()).try_into().unwrap()) == Some(M)
    )) && ((
        crossword.grid().get((pos + NorthEast.delta()).try_into().unwrap()) == Some(M) &&
        crossword.grid().get((pos + SouthWest.delta()).try_into().unwrap()) == Some(S)
    ) || (
        crossword.grid().get((pos + NorthEast.delta()).try_into().unwrap()) == Some(S) &&
        crossword.grid().get((pos + SouthWest.delta()).try_into().unwrap()) == Some(M)
    ));
    println!("{}", valid);

    // let pos = Vec2USize(2, 1);
    // let asdf = (crossword.grid().get(pos) == Some(A)) && ((
    //     if let Ok(offset) = NorthWest.delta().try_into() { crossword.grid().get(pos + offset) == Some(M) } else { false } &&
    //     if let Ok(offset) = SouthEast.delta().try_into() { crossword.grid().get(pos + offset) == Some(S) } else { false }
    // ) || (
    //     if let Ok(offset) = NorthWest.delta().try_into() { crossword.grid().get(pos + offset) == Some(S) } else { false } &&
    //     if let Ok(offset) = SouthEast.delta().try_into() { crossword.grid().get(pos + offset) == Some(M) } else { false }
    // ));
    // println!("{}", asdf);

    Ok(())
}
