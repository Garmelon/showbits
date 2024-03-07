use taffy::{AvailableSpace, Size};

use crate::{Node, View};

pub trait Widget<C> {
    #[allow(unused_variables)]
    fn size(
        &mut self,
        ctx: &mut C,
        known: Size<Option<f32>>,
        available: Size<AvailableSpace>,
    ) -> Size<f32> {
        Size::ZERO
    }

    fn draw_below(&mut self, ctx: &mut C, view: &mut View<'_>) -> anyhow::Result<()>;
    fn draw_above(&mut self, ctx: &mut C, view: &mut View<'_>) -> anyhow::Result<()>;
}

pub type BoxedWidget<C> = Box<dyn Widget<C>>;

pub trait WidgetExt<C> {
    fn node(self) -> Node<C>;
}

impl<C, W: Widget<C> + 'static> WidgetExt<C> for W {
    fn node(self) -> Node<C> {
        Node::empty().widget(self)
    }
}
