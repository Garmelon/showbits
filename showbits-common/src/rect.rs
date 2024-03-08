use std::ops::{Add, Sub};

use crate::Vec2;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    north: i32,
    south: i32,
    west: i32,
    east: i32,
}

impl Rect {
    /// A zero-sized rectangle located at the origin.
    pub const ZERO: Self = Self::new(0, 0, -1, -1);

    /// Whenever a rectangle is constructed, this function must be used. This
    /// ensures invariants are always checked.
    const fn new(north: i32, south: i32, west: i32, east: i32) -> Self {
        let result = Self {
            north,
            south,
            west,
            east,
        };

        let size = result.size();
        assert!(size.x >= 0);
        assert!(size.y >= 0);

        result
    }

    /// Construct a rectangle that is a bounding box around two points.
    ///
    /// It is not possible to construct a rectangle with a width or height of 0
    /// through this method. Use one of the other constructor functions instead.
    pub fn from_points(a: Vec2, b: Vec2) -> Self {
        Self::new(a.y.min(b.y), a.y.max(b.y), a.x.min(b.x), a.x.max(b.x))
    }

    /// Construct a rectangle from its north-west and south-east corners.
    pub const fn from_nw_se(nw: Vec2, se: Vec2) -> Self {
        Self::new(nw.y, se.y, nw.x, se.x)
    }

    /// Construct a rectangle from its north-east and south-west corners.
    pub const fn from_ne_sw(ne: Vec2, sw: Vec2) -> Self {
        Self::new(ne.y, sw.y, sw.x, ne.x)
    }

    /// Construct a rectangle from its north-west corner and size.
    pub fn from_nw(nw: Vec2, size: Vec2) -> Self {
        let se = nw + (size - 1);
        Self::from_nw_se(nw, se)
    }

    /// Construct a rectangle from its north-east corner and size.
    pub fn from_ne(ne: Vec2, size: Vec2) -> Self {
        let sw = ne + (size - 1).neg_x();
        Self::from_ne_sw(ne, sw)
    }

    /// Construct a rectangle from its south-west corner and size.
    pub fn from_sw(sw: Vec2, size: Vec2) -> Self {
        let ne = sw + (size - 1).neg_y();
        Self::from_ne_sw(ne, sw)
    }

    /// Construct a rectangle from its south-east corner and size.
    pub fn from_se(se: Vec2, size: Vec2) -> Self {
        let nw = se - (size - 1);
        Self::from_nw_se(nw, se)
    }

    pub const fn corner_nw(self) -> Vec2 {
        Vec2::new(self.west, self.north)
    }

    pub const fn corner_ne(self) -> Vec2 {
        Vec2::new(self.east, self.north)
    }

    pub const fn corner_sw(self) -> Vec2 {
        Vec2::new(self.west, self.south)
    }

    pub const fn corner_se(self) -> Vec2 {
        Vec2::new(self.east, self.south)
    }

    pub const fn size(self) -> Vec2 {
        Vec2::new(self.east - self.west + 1, self.south - self.north + 1)
    }

    /// An iterator of all points contained within the rectangle.
    pub fn points(self) -> impl Iterator<Item = Vec2> {
        (self.north..=self.south)
            .flat_map(move |y| (self.west..=self.east).map(move |x| Vec2::new(x, y)))
    }
}

impl Add<Vec2> for Rect {
    type Output = Self;

    fn add(self, rhs: Vec2) -> Self::Output {
        Self::new(
            self.north + rhs.y,
            self.south + rhs.y,
            self.west + rhs.x,
            self.east + rhs.x,
        )
    }
}

impl Sub<Vec2> for Rect {
    type Output = Self;

    fn sub(self, rhs: Vec2) -> Self::Output {
        Self::new(
            self.north - rhs.y,
            self.south - rhs.y,
            self.west - rhs.x,
            self.east - rhs.x,
        )
    }
}
