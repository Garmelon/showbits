use taffy::{AvailableSpace, Layout, Size};

use crate::{Node, View};

pub trait Widget<C> {
    /// Used for the measure function in
    /// [`taffy::TaffyTree::compute_layout_with_measure`].
    #[allow(unused_variables)]
    fn size(
        &mut self,
        ctx: &mut C,
        known: Size<Option<f32>>,
        available: Size<AvailableSpace>,
    ) -> Size<f32> {
        Size::ZERO
    }

    /// Called before all children are drawn.
    ///
    /// Prefer this over [`Self::draw_above`] when implementing a leaf widget.
    #[allow(unused_variables)]
    fn draw_below(
        &mut self,
        ctx: &mut C,
        view: &mut View<'_>,
        layout: &Layout,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    /// Called after all children are drawn.
    #[allow(unused_variables)]
    fn draw_above(
        &mut self,
        ctx: &mut C,
        view: &mut View<'_>,
        layout: &Layout,
    ) -> anyhow::Result<()> {
        Ok(())
    }
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
