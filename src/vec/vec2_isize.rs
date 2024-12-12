use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy)]
pub struct Vec2ISize(pub isize, pub isize);

impl Add for Vec2ISize {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Mul::<isize> for Vec2ISize {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}
