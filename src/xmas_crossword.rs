use std::{fmt::Display, io::repeat};
use utf8_box_builder::*;

use crate::xmas_letter::{NonXmasLetterError, XmasLetter};

#[derive(Debug)] pub struct BadInputError;

impl From::<NonXmasLetterError> for BadInputError {
    fn from(value: NonXmasLetterError) -> Self {
        Self
    }
}

pub struct XmasCrossword(pub Vec<Vec<XmasLetter>>);

impl Display for XmasCrossword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // top row
        write!(f, "{TLC}{HL}")?;
        self.0.iter().for_each(|_| write!(f, "{HL}{HL}").unwrap());
        writeln!(f, "{TRC}")?;

        // middle rows
        self.0.iter().for_each(|row| {
            write!(f, "{VL} ").unwrap();
            row.iter().for_each(|letter| write!(f, "{letter} ").unwrap());
            writeln!(f, "{VL}").unwrap();
        });

        // bottom row
        write!(f, "{BLC}{HL}")?;
        self.0.iter().for_each(|_| write!(f, "{HL}{HL}").unwrap());
        writeln!(f, "{BRC}")?;

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
