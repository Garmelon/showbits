use palette::Srgba;
use taffy::Layout;

use crate::{Rect, Vec2, View, Widget, color};

pub struct Block {
    border: Srgba,
    background: Srgba,
}

impl Block {
    pub fn new() -> Self {
        Self {
            border: color::TRANSPARENT,
            background: color::TRANSPARENT,
        }
    }

    pub fn with_border(mut self, color: Srgba) -> Self {
        self.border = color;
        self
    }

    pub fn with_background(mut self, color: Srgba) -> Self {
        self.background = color;
        self
    }
}

impl Default for Block {
    fn default() -> Self {
        Self::new()
    }
}

impl<C> Widget<C> for Block {
    fn draw_below(
        &mut self,
        _ctx: &mut C,
        view: &mut View<'_>,
        layout: &Layout,
    ) -> anyhow::Result<()> {
        let area = view.area();

        // Background
        view.rect(area, self.background);

        // And now... the border!
        //
        // It is important not to draw pixels twice in case the border color is
        // transparent. That's why the logic below is a bit more complex.
        let left = layout.border.left as i32;
        let right = layout.border.right as i32;
        let top = layout.border.top as i32;
        let bottom = layout.border.bottom as i32;
        if top > 0 {
            let border = Rect::from_nw(area.corner_nw(), area.size().with_y(top));
            view.rect(border, self.border);
        }
        if bottom > 0 {
            let border = Rect::from_sw(area.corner_sw(), area.size().with_y(bottom));
            view.rect(border, self.border);
        }
        if left > 0 {
            let nw = area.corner_nw() + Vec2::new(0, top);
            let size = Vec2::new(left, area.size().y - top - bottom);
            view.rect(Rect::from_nw(nw, size), self.border);
        }
        if right > 0 {
            let ne = area.corner_ne() + Vec2::new(0, top);
            let size = Vec2::new(right, area.size().y - top - bottom);
            view.rect(Rect::from_ne(ne, size), self.border);
        }

        Ok(())
    }
}
