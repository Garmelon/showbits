use crate::{Rect, Vec2, View};

pub trait Widget<C> {
    /// Size that the widget wants to be, given the width and height
    /// constraints.
    fn size(&self, max_width: Option<i32>, max_height: Option<i32>) -> Vec2;

    /// Recalculate the size of all inner widgets given the widget's own size.
    ///
    /// # Implement if...
    ///
    /// - There are inner widgets
    fn resize(&mut self, _area: Rect) {}

    /// Perform any updates (e.g. fetching map tiles) that require the widget's
    /// size and may fail.
    ///
    /// # Implement if...
    ///
    /// - There are inner widgets
    /// - This widget needs to perform updates
    fn update(&mut self, _area: Rect) -> anyhow::Result<()> {
        Ok(())
    }

    fn draw(self, view: &mut View<'_, C>);
}

/// Wrapper trait around [`Widget`] that turns `Box<Self>` into a `Self` to get
/// around the "size cannot be statically determined" error with the naïve
/// approach of `Box<Widget>`.
trait WidgetWrapper<C> {
    fn size(&self, max_width: Option<i32>, max_height: Option<i32>) -> Vec2;
    fn resize(&mut self, _area: Rect);
    fn update(&mut self, _area: Rect) -> anyhow::Result<()>;
    fn draw(self: Box<Self>, view: &mut View<'_, C>);
}

impl<C, W: Widget<C>> WidgetWrapper<C> for W {
    // These implementations explicitly use `Widget::*` to call the widget
    // methods even though they have priority over the `WidgetWrapper::*`
    // methods for some reason. Just a bit of rustc magic, I guess.

    fn size(&self, max_width: Option<i32>, max_height: Option<i32>) -> Vec2 {
        Widget::size(self, max_width, max_height)
    }

    fn resize(&mut self, area: Rect) {
        Widget::resize(self, area);
    }

    fn update(&mut self, area: Rect) -> anyhow::Result<()> {
        Widget::update(self, area)
    }

    fn draw(self: Box<Self>, view: &mut View<'_, C>) {
        Widget::draw(*self, view);
    }
}

pub struct BoxedWidget<C> {
    area: Rect,
    widget: Box<dyn WidgetWrapper<C>>,
}

impl<C> BoxedWidget<C> {
    pub fn new<W>(widget: W) -> Self
    where
        W: Widget<C> + 'static,
    {
        Self {
            area: Rect::ZERO,
            widget: Box::new(widget),
        }
    }

    pub fn area(&self) -> Rect {
        self.area
    }
}

impl<C> Widget<C> for BoxedWidget<C> {
    fn size(&self, max_width: Option<i32>, max_height: Option<i32>) -> Vec2 {
        self.widget.size(max_width, max_height)
    }

    fn resize(&mut self, area: Rect) {
        self.widget.resize(area);
    }

    fn update(&mut self, area: Rect) -> anyhow::Result<()> {
        self.widget.update(area)
    }

    fn draw(self, view: &mut View<'_, C>) {
        self.widget.draw(view);
    }
}
