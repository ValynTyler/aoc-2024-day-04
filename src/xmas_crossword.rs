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
        write!(f, "┌─")?;
        for _ in 0..self.0.len() - 1 {
            write!(f, "──")?;
        }
        writeln!(f, "──┐")?;
        for row in &self.0 {
            write!(f, "│ ")?;
            for item in row  {
                write!(f, "{} ", item)?;
            }
            writeln!(f, "│")?;
        }
        write!(f, "└─")?;
        for _ in 0..self.0.len() - 1 {
            write!(f, "──")?;
        }
        write!(f, "──┘")?;
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
