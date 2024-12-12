use std::ops::Add;

pub struct Vec2USize(pub usize, pub usize);

impl Add for Vec2USize {
    type Output = Self;

    fn add(self, rhs: Vec2USize) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}
