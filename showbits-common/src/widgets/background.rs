use crate::{BoxedWidget, Color, Vec2, Widget};

pub struct Background<C> {
    inner: BoxedWidget<C>,
    color: C,
}

impl<C> Background<C> {
    pub fn new(inner: BoxedWidget<C>, color: C) -> Self {
        Self { inner, color }
    }
}

impl<C: Color> Widget<C> for Background<C> {
    fn size(&self, max_width: Option<i32>, max_height: Option<i32>) -> crate::Vec2 {
        self.inner.size(max_width, max_height)
    }

    fn resize(&mut self, area: crate::Rect) {
        self.inner.resize(area);
    }

    fn update(&mut self, _area: crate::Rect) -> anyhow::Result<()> {
        Ok(())
    }

    fn draw(self, view: &mut crate::View<'_, C>) {
        for y in 0..view.size().y {
            for x in 0..view.size().x {
                view.set(Vec2::new(x, y), self.color);
            }
        }
    }
}
