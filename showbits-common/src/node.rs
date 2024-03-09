use paste::paste;
use taffy::{
    AlignContent, AlignItems, AlignSelf, Dimension, Display, FlexDirection, FlexWrap, GridAutoFlow,
    GridPlacement, JustifyContent, LengthPercentage, LengthPercentageAuto, Line, NodeId,
    NonRepeatedTrackSizingFunction, Overflow, Point, Position, Rect, Size, Style, TaffyResult,
    TrackSizingFunction,
};

use crate::{BoxedWidget, Tree, Widget};

pub struct Node<C> {
    layout: Style,
    children: Vec<NodeId>,
    widget: Option<BoxedWidget<C>>,
}

impl<C> Node<C> {
    pub fn empty() -> Self {
        Self {
            layout: Style::default(),
            children: vec![],
            widget: None,
        }
    }

    pub fn and_child(mut self, node: NodeId) -> Self {
        self.children.push(node);
        self
    }

    pub fn with_widget<W: Widget<C> + 'static>(mut self, widget: W) -> Self {
        self.widget = Some(Box::new(widget));
        self
    }

    pub fn register(self, tree: &mut Tree<C>) -> TaffyResult<NodeId> {
        let tree = tree.taffy_tree();
        let id = tree.new_with_children(self.layout, &self.children)?;
        tree.set_node_context(id, self.widget)?;
        Ok(id)
    }
}

// Layout helper functions

macro_rules! layout_setter {
    ( $name:ident : $type:ty ) => {
        paste! {
            pub fn [<with_ $name>](mut self, $name: $type) -> Self {
                self.layout.$name = $name;
                self
            }
        }
    };
}

macro_rules! layout_setter_point {
    ( $name:ident : Point<$type:ty> ) => {
        paste! {
            pub fn [<with_ $name>](mut self, $name: Point<$type>) -> Self {
                self.layout.$name = $name;
                self
            }

            pub fn [<with_ $name _x>](mut self, x: $type) -> Self {
                self.layout.$name.x = x;
                self
            }

            pub fn [<with_ $name _y>](mut self, y: $type) -> Self {
                self.layout.$name.y = y;
                self
            }

            pub fn [<with_ $name _all>](mut self, all: $type) -> Self {
                self.layout.$name.x = all;
                self.layout.$name.y = all;
                self
            }
        }
    };
}

macro_rules! layout_setter_size {
    ( $name:ident : Size<$type:ty> ) => {
        paste! {
            pub fn [<with_ $name>](mut self, $name: Size<$type>) -> Self {
                self.layout.$name = $name;
                self
            }

            pub fn [<with_ $name _width>](mut self, width: $type) -> Self {
                self.layout.$name.width = width;
                self
            }

            pub fn [<with_ $name _height>](mut self, height: $type) -> Self {
                self.layout.$name.height = height;
                self
            }

            pub fn [<with_ $name _all>](mut self, all: $type) -> Self {
                self.layout.$name.width = all;
                self.layout.$name.height = all;
                self
            }
        }
    };
}

macro_rules! layout_setter_line {
    ( $name:ident : Line<$type:ty> ) => {
        paste! {
            pub fn [<with_ $name>](mut self, $name: Line<$type>) -> Self {
                self.layout.$name = $name;
                self
            }

            pub fn [<with_ $name _start>](mut self, start: $type) -> Self {
                self.layout.$name.start = start;
                self
            }

            pub fn [<with_ $name _end>](mut self, end: $type) -> Self {
                self.layout.$name.end = end;
                self
            }

            pub fn [<with_ $name _all>](mut self, all: $type) -> Self {
                self.layout.$name.start = all;
                self.layout.$name.end = all;
                self
            }
        }
    };
}

macro_rules! layout_setter_rect {
    ( $name:ident : Rect<$type:ty> ) => {
        paste! {
            pub fn [<with_ $name>](mut self, $name: Rect<$type>) -> Self {
                self.layout.$name = $name;
                self
            }

            pub fn [<with_ $name _left>](mut self, left: $type) -> Self {
                self.layout.$name.left = left;
                self
            }

            pub fn [<with_ $name _right>](mut self, right: $type) -> Self {
                self.layout.$name.right = right;
                self
            }

            pub fn [<with_ $name _top>](mut self, top: $type) -> Self {
                self.layout.$name.top = top;
                self
            }

            pub fn [<with_ $name _bottom>](mut self, left: $type) -> Self {
                self.layout.$name.bottom = left;
                self
            }

            pub fn [<with_ $name _horiz>](mut self, horizontal: $type) -> Self {
                self.layout.$name.left = horizontal;
                self.layout.$name.right = horizontal;
                self
            }

            pub fn [<with_ $name _vert>](mut self, vertical: $type) -> Self {
                self.layout.$name.top = vertical;
                self.layout.$name.bottom = vertical;
                self
            }

            pub fn [<with_ $name _all>](mut self, all: $type) -> Self {
                self.layout.$name.left = all;
                self.layout.$name.right = all;
                self.layout.$name.top = all;
                self.layout.$name.bottom = all;
                self
            }
        }
    };
}

impl<C> Node<C> {
    layout_setter!(display: Display);
    layout_setter_point!(overflow: Point<Overflow>);
    layout_setter!(scrollbar_width: f32);
    layout_setter!(position: Position);
    layout_setter_rect!(inset: Rect<LengthPercentageAuto>);
    layout_setter_size!(size: Size<Dimension>);
    layout_setter_size!(min_size: Size<Dimension>);
    layout_setter_size!(max_size: Size<Dimension>);
    layout_setter!(aspect_ratio: Option<f32>);
    layout_setter_rect!(margin: Rect<LengthPercentageAuto>);
    layout_setter_rect!(padding: Rect<LengthPercentage>);
    layout_setter_rect!(border: Rect<LengthPercentage>);
    layout_setter!(align_items: Option<AlignItems>);
    layout_setter!(align_self: Option<AlignSelf>);
    layout_setter!(justify_items: Option<AlignItems>);
    layout_setter!(justify_self: Option<AlignSelf>);
    layout_setter!(align_content: Option<AlignContent>);
    layout_setter!(justify_content: Option<JustifyContent>);
    layout_setter_size!(gap: Size<LengthPercentage>);
    layout_setter!(flex_direction: FlexDirection);
    layout_setter!(flex_wrap: FlexWrap);
    layout_setter!(flex_basis: Dimension);
    layout_setter!(flex_grow: f32);
    layout_setter!(flex_shrink: f32);
    layout_setter!(grid_template_rows: Vec<TrackSizingFunction>);
    layout_setter!(grid_template_columns: Vec<TrackSizingFunction>);
    layout_setter!(grid_auto_rows: Vec<NonRepeatedTrackSizingFunction>);
    layout_setter!(grid_auto_columns: Vec<NonRepeatedTrackSizingFunction>);
    layout_setter!(grid_auto_flow: GridAutoFlow);
    layout_setter_line!(grid_row: Line<GridPlacement>);
    layout_setter_line!(grid_column: Line<GridPlacement>);
}
