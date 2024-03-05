use image::RgbImage;
use palette::Srgb;

use crate::{Rect, Vec2};

pub struct View<'a> {
    area: Rect,
    buffer: &'a mut RgbImage,
}

impl<'a> View<'a> {
    pub fn new(buffer: &'a mut RgbImage) -> Self {
        let size = Vec2::from_u32(buffer.width(), buffer.height());
        let area = Rect::from_nw(Vec2::ZERO, size);
        Self { area, buffer }
    }

    pub fn dup(&mut self) -> View<'_> {
        View {
            area: self.area,
            buffer: self.buffer,
        }
    }

    pub fn zoom(mut self, area: Rect) -> Self {
        self.area = area + self.area.corner_nw();
        self
    }

    pub fn size(&self) -> Vec2 {
        self.area.size()
    }

    fn pos_to_buffer_pos(&self, pos: Vec2) -> Vec2 {
        pos + self.area.corner_nw()
    }

    pub fn get(&self, pos: Vec2) -> Option<Srgb> {
        let (x, y) = self.pos_to_buffer_pos(pos).to_u32();
        let pixel = self.buffer.get_pixel_checked(x, y)?;
        let [r, g, b] = pixel.0;
        let color = Srgb::new(r, g, b);
        Some(color.into_format())
    }

    pub fn set(&mut self, pos: Vec2, color: Srgb) {
        let (x, y) = self.pos_to_buffer_pos(pos).to_u32();
        if let Some(pixel) = self.buffer.get_pixel_mut_checked(x, y) {
            let color = color.into_format::<u8>();
            pixel.0 = [color.red, color.green, color.blue];
        }
    }
}
