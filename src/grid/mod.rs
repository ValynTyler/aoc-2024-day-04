pub mod grid_direction;
use std::fmt::Display;
use utf8_box_builder::*;

pub use grid_direction::*;

pub struct Grid<T>(pub Vec<Vec<T>>);

impl<T> Display for Grid<T>
where T: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // top row
        write!(f, "{TLC}{HL}")?;
        self.0[0].iter().for_each(|_| write!(f, "{HL}{HL}").unwrap());
        writeln!(f, "{TRC}")?;

        // middle rows
        self.0.iter().for_each(|row| {
            write!(f, "{VL} ").unwrap();
            row.iter().for_each(|item| {
                write!(f, "{item} ").unwrap()
            });
            writeln!(f, "{VL}").unwrap();
        });

        // bottom row
        write!(f, "{BLC}{HL}")?;
        self.0[0].iter().for_each(|_| write!(f, "{HL}{HL}").unwrap());
        write!(f, "{BRC}")?;

        Ok(())
    }
}
