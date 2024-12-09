use crate::xmas_letter::{NonXmasLetterError, XmasLetter};

pub struct BadInputError;

impl From::<NonXmasLetterError> for BadInputError {
    fn from(value: NonXmasLetterError) -> Self {
        Self
    }
}

pub struct PuzzleInput(Vec<Vec<XmasLetter>>);

impl TryFrom::<&str> for PuzzleInput {
    type Error = BadInputError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let v: Vec<Vec<XmasLetter>> = value
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.try_into())
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<Vec<XmasLetter>>, NonXmasLetterError>>()?;

        todo!()
    }
}
