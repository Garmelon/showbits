use taffy::{NodeId, Style, TaffyResult, TaffyTree};

use crate::Widget;

pub struct Node {
    layout: Style,
    children: Vec<NodeId>,
    widget: Option<Box<dyn Widget>>,
}

impl Node {
    pub fn empty() -> Self {
        Self {
            layout: Style::default(),
            children: vec![],
            widget: None,
        }
    }

    pub fn widget<W: Widget + 'static>(mut self, widget: W) -> Self {
        self.widget = Some(Box::new(widget));
        self
    }

    pub fn register(self, tree: &mut TaffyTree<Box<dyn Widget>>) -> TaffyResult<NodeId> {
        let id = tree.new_with_children(self.layout, &self.children)?;
        tree.set_node_context(id, self.widget)?;
        Ok(id)
    }
}
