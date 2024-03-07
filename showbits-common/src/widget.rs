use taffy::{AvailableSpace, Size};

use crate::{Node, View};

pub trait Widget {
    #[allow(unused_variables)]
    fn size(&mut self, known: Size<Option<f32>>, available: Size<AvailableSpace>) -> Size<f32> {
        Size::ZERO
    }

    fn draw_below(&mut self, view: &mut View<'_>) -> anyhow::Result<()>;
    fn draw_above(&mut self, view: &mut View<'_>) -> anyhow::Result<()>;
}

pub trait WidgetExt {
    fn node(self) -> Node;
}

impl<W: Widget + 'static> WidgetExt for W {
    fn node(self) -> Node {
        Node::empty().widget(self)
    }
}
