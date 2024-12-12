use std::fmt::Display;

use crate::grid::Grid;

use super::{NonXmasLetterError, XmasLetter};

#[derive(Debug)]
pub struct BadDataError;

impl From::<NonXmasLetterError> for BadDataError {
    fn from(_: NonXmasLetterError) -> Self { Self }
}

pub struct XmasGrid(Grid<XmasLetter>);

impl Display for XmasGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom::<&str> for XmasGrid {
    type Error = BadDataError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(value
            .trim()
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| XmasLetter::try_from(c))
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()
            .and_then(|mat| Ok(Self(Grid(mat))))?
        )
    }
}
