use crate::{Buffer, Rect, Vec2};

// TODO Add Orientation (from inkfo)

pub struct View<'a, C> {
    area: Rect,
    buffer: &'a mut Buffer<C>,
}

impl<'a, C> View<'a, C> {
    pub fn new(buffer: &'a mut Buffer<C>) -> Self {
        Self {
            area: Rect::from_nw(Vec2::ZERO, buffer.size()),
            buffer,
        }
    }

    pub fn dup(&mut self) -> View<'_, C> {
        View {
            area: self.area,
            buffer: self.buffer,
        }
    }

    pub fn with_area(mut self, area: Rect) -> Self {
        self.area = area;
        self
    }

    pub fn translated(self, delta: Vec2) -> Self {
        let area = self.area + delta;
        self.with_area(area)
    }

    pub fn size(&self) -> Vec2 {
        self.area.size()
    }

    fn pos_to_buffer_pos(&self, pos: Vec2) -> Vec2 {
        pos + self.area.corner_nw()
    }

    pub fn at(&self, pos: Vec2) -> Option<&C> {
        self.buffer.at(self.pos_to_buffer_pos(pos))
    }

    pub fn set(&mut self, pos: Vec2, color: C) {
        if let Some(pixel) = self.buffer.at_mut(self.pos_to_buffer_pos(pos)) {
            *pixel = color;
        }
    }
}
