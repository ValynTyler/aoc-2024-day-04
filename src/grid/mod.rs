pub mod grid_direction;
pub use grid_direction::*;

use std::fmt::Display;
use utf8_box_builder::*;

use crate::vec::Vec2USize;

pub struct Grid<T>(pub Vec<Vec<T>>);

impl<T> Display for Grid<T>
where T: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // top row
        write!(f, "{TLC}{HL}")?;
        let res: Result<Vec<_>, _> = self.0[0].iter().map(|_| write!(f, "{HL}{HL}")).collect();
        res?;
        writeln!(f, "{TRC}")?;

        // middle rows
        let _: Result<Vec<_>, _> = self.0.iter().map(|row| {
            write!(f, "{VL} ")?;
            let res: Result<Vec<_>, _> = row.iter().map(|item| {
                write!(f, "{item} ")
            }).collect();
            res?;
            writeln!(f, "{VL}")
        }).collect();

        // bottom row
        write!(f, "{BLC}{HL}")?;
        let res: Result<Vec<_>, _> = self.0[0].iter().map(|_| write!(f, "{HL}{HL}")).collect();
        res?;
        write!(f, "{BRC}")?;

        Ok(())
    }
}

impl<T> Grid<T>
where T: Copy {
    pub fn get(&self, pos: Vec2USize) -> Option<T> {
        if let Some(row) = self.0.get(pos.1) {
            if let Some(item) = row.get(pos.0) {
                return Some(*item)
            }
        }

        None
    }
}
