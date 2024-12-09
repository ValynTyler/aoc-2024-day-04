use std::fmt::Display;
use utf8_box_builder::*;

use aoc_2024_day4::xmas_crossword::XmasCrossword;

use crate::xmas_direction::XmasDirection;

pub struct SolvedCrossword(Vec<Vec<Option<XmasDirection>>>);

impl Display for SolvedCrossword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // top row
        write!(f, "{TLC}{HL}")?;
        self.0.iter().for_each(|_| write!(f, "{HL}{HL}").unwrap());
        writeln!(f, "{TRC}")?;

        // middle rows
        self.0.iter().for_each(|row| {
            write!(f, "{VL} ").unwrap();
            row.iter().for_each(|letter| {
                let symbol = if letter.is_some() { 'A' } else { '.' };
                write!(f, "{symbol} ").unwrap()
            });
            writeln!(f, "{VL}").unwrap();
        });

        // bottom row
        write!(f, "{BLC}{HL}")?;
        self.0.iter().for_each(|_| write!(f, "{HL}{HL}").unwrap());
        write!(f, "{BRC}")?;

        Ok(())
    }
}

impl From::<XmasCrossword> for SolvedCrossword {
    fn from(value: XmasCrossword) -> Self {
        Self(value.0
            .iter()
            .map(|row| {
                row
                    .iter()
                    .map(|_| {
                        None
                    })
                    .collect()
            })
            .collect()
        )
    }
}
