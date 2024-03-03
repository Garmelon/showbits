use crate::{Vec2, View, Widget};

pub struct Empty {}

impl Empty {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Empty {
    fn default() -> Self {
        Self::new()
    }
}

impl<C> Widget<C> for Empty {
    fn size(&self, _max_width: Option<i32>, _max_height: Option<i32>) -> Vec2 {
        Vec2::ZERO
    }

    fn draw(self, _view: &mut View<'_, C>) {}
}
