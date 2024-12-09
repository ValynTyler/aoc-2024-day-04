use aoc_2024_day4::xmas_crossword::XmasCrossword;

use crate::xmas_direction::XmasDirection;

pub struct SolvedCrossword(Vec<Vec<Option<XmasDirection>>>);

impl From::<XmasCrossword> for SolvedCrossword {
    fn from(value: XmasCrossword) -> Self {
        todo!()
    }
}
