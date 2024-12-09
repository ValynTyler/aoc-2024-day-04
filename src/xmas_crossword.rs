use std::{fmt::Display, io::repeat};

use crate::xmas_letter::{NonXmasLetterError, XmasLetter};

#[derive(Debug)] pub struct BadInputError;

impl From::<NonXmasLetterError> for BadInputError {
    fn from(value: NonXmasLetterError) -> Self {
        Self
    }
}

pub struct XmasCrossword(Vec<Vec<XmasLetter>>);

impl Display for XmasCrossword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // top row
        write!(f, "┌─")?;
        self.0.iter().for_each(|_| write!(f, "──").unwrap());
        writeln!(f, "┐")?;

        // middle rows
        self.0.iter().for_each(|row| {
            write!(f, "│ ").unwrap();
            row.iter().for_each(|letter| write!(f, "{letter} ").unwrap());
            writeln!(f, "│").unwrap();
        });

        // bottom row
        write!(f, "└─")?;
        self.0.iter().for_each(|_| write!(f, "──").unwrap());
        writeln!(f, "┘")?;

        Ok(())
    }
}

impl TryFrom::<&str> for XmasCrossword {
    type Error = BadInputError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self(value
            .trim()
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.try_into())
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<Vec<XmasLetter>>, NonXmasLetterError>>()?
        ))
    }
}
