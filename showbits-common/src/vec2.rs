use std::{
    fmt,
    ops::{Add, Mul, Neg, Sub},
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    pub const ZERO: Self = Self::new(0, 0);
    pub const NORTH: Self = Self::new(0, -1);
    pub const SOUTH: Self = Self::new(0, 1);
    pub const WEST: Self = Self::new(-1, 0);
    pub const EAST: Self = Self::new(1, 0);

    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn from_u32_checked(x: u32, y: u32) -> Option<Self> {
        let x: i32 = x.try_into().ok()?;
        let y: i32 = y.try_into().ok()?;
        Some(Self::new(x, y))
    }

    pub fn from_u32(x: u32, y: u32) -> Self {
        let x: i32 = x.try_into().expect("x too large");
        let y: i32 = y.try_into().expect("y too large");
        Self::new(x, y)
    }

    pub fn to_u32_checked(self) -> Option<(u32, u32)> {
        let x: u32 = self.x.try_into().ok()?;
        let y: u32 = self.y.try_into().ok()?;
        Some((x, y))
    }

    pub fn to_u32(self) -> (u32, u32) {
        let x: u32 = self.x.try_into().expect("x too small");
        let y: u32 = self.y.try_into().expect("y too small");
        (x, y)
    }

    pub fn with_x(self, x: i32) -> Self {
        Self { x, ..self }
    }

    pub fn with_y(self, y: i32) -> Self {
        Self { y, ..self }
    }

    /// The vector pointing from `self` to `other`.
    ///
    /// ```
    /// # use showbits_common::Vec2;
    /// let a = Vec2::new(1, 3);
    /// let b = Vec2::new(3, 7);
    /// assert_eq!(a.to(b), b - a);
    /// ```
    pub fn to(self, other: Self) -> Self {
        other - self
    }

    /// Negate the `x` component of the vector.
    ///
    /// ```
    /// # use showbits_common::Vec2;
    /// let v = Vec2::new(3, 4);
    /// assert_eq!(v.neg_x(), v * Vec2::new(-1, 1));
    /// ```
    pub fn neg_x(self) -> Self {
        Self { x: -self.x, ..self }
    }

    /// Negate the `y` component of the vector.
    ///
    /// ```
    /// # use showbits_common::Vec2;
    /// let v = Vec2::new(3, 4);
    /// assert_eq!(v.neg_y(), v * Vec2::new(1, -1));
    /// ```
    pub fn neg_y(self) -> Self {
        Self { y: -self.y, ..self }
    }
}

impl fmt::Debug for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Vec2").field(&self.x).field(&self.y).finish()
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<i32> for Vec2 {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        self + Self::new(rhs, rhs)
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<i32> for Vec2 {
    type Output = Self;

    fn sub(self, rhs: i32) -> Self::Output {
        self - Self::new(rhs, rhs)
    }
}

impl Mul for Vec2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Mul<i32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        self * Self::new(rhs, rhs)
    }
}

#[cfg(test)]
mod tests {
    use crate::Vec2;

    #[test]
    fn arithmetic() {
        let a = Vec2::new(1, 3);
        let b = Vec2::new(3, 7);

        assert_eq!(-a, Vec2::new(-1, -3));
        assert_eq!(a.neg_x(), Vec2::new(-1, 3));
        assert_eq!(a.neg_y(), Vec2::new(1, -3));

        assert_eq!(a + b, Vec2::new(4, 10));
        assert_eq!(a + 2, Vec2::new(3, 5));

        assert_eq!(a - b, Vec2::new(-2, -4));
        assert_eq!(a - 2, Vec2::new(-1, 1));
        assert_eq!(a - b, b.to(a));

        assert_eq!(a * b, Vec2::new(3, 21));
        assert_eq!(a * 2, Vec2::new(2, 6));
    }
}
