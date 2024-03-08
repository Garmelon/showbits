use image::RgbImage;
use palette::Srgb;

use crate::{color, Rect, Vec2};

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
        Some(color::from_image_rgb(*pixel))
    }

    pub fn set(&mut self, pos: Vec2, color: Srgb) {
        let (x, y) = self.pos_to_buffer_pos(pos).to_u32();
        if let Some(pixel) = self.buffer.get_pixel_mut_checked(x, y) {
            *pixel = color::to_image_rgb(color);
        }
    }

    // More complicated drawing primitives

    pub fn rect(&mut self, area: Rect, color: Srgb) {
        let nw = area.corner_nw();
        let se = area.corner_se();
        for y in nw.y..=se.y {
            for x in nw.x..=se.x {
                self.set(Vec2::new(x, y), color);
            }
        }
    }
}
