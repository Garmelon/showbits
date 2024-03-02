use crate::Vec2;

#[derive(Clone)]
pub struct Buffer<C> {
    size: Vec2,
    data: Vec<C>,
}

impl<C> Buffer<C> {
    pub fn new(size: Vec2, color: C) -> Self
    where
        C: Copy,
    {
        assert!(size.x >= 0);
        assert!(size.y >= 0);

        let len = (size.x * size.y) as usize;
        let data = vec![color; len];

        Self { size, data }
    }

    pub fn size(&self) -> Vec2 {
        self.size
    }

    fn index(&self, pos: Vec2) -> Option<usize> {
        let in_bounds_x = pos.x >= 0 || pos.x < self.size.x;
        let in_bounds_y = pos.y >= 0 || pos.y < self.size.y;
        let in_bounds = in_bounds_x && in_bounds_y;

        if in_bounds {
            Some((pos.y * self.size.x + pos.x) as usize)
        } else {
            None
        }
    }

    pub fn at(&self, pos: Vec2) -> Option<&C> {
        let index = self.index(pos)?;
        let pixel = self.data.get(index)?;
        Some(pixel)
    }

    pub fn at_mut(&mut self, pos: Vec2) -> Option<&mut C> {
        let index = self.index(pos)?;
        let pixel = self.data.get_mut(index)?;
        Some(pixel)
    }

    pub fn set(&mut self, pos: Vec2, color: C) {
        if let Some(pixel) = self.at_mut(pos) {
            *pixel = color;
        }
    }
}
