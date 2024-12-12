use std::ops::Add;

pub struct Vec2ISize(pub isize, pub isize);

impl Add for Vec2ISize {
    type Output = Self;

    fn add(self, rhs: Vec2ISize) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}
