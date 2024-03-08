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

    pub fn child(mut self, node: NodeId) -> Self {
        self.children.push(node);
        self
    }

    pub fn widget<W: Widget<C> + 'static>(mut self, widget: W) -> Self {
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
        pub fn $name(mut self, $name: $type) -> Self {
            self.layout.$name = $name;
            self
        }
    };
}

macro_rules! layout_setter_point {
    (
        $name:ident : Point<$type:ty>
        as $name_x:ident, $name_y:ident
        as $name_all:ident
    ) => {
        pub fn $name_x(mut self, x: $type) -> Self {
            self.layout.$name.x = x;
            self
        }

        pub fn $name_y(mut self, y: $type) -> Self {
            self.layout.$name.y = y;
            self
        }

        pub fn $name_all(mut self, all: $type) -> Self {
            self.layout.$name.x = all;
            self.layout.$name.y = all;
            self
        }
    };
}

macro_rules! layout_setter_size {
    (
        $name:ident : Size<$type:ty>
        as $name_width:ident, $name_height:ident
        as $name_all:ident
    ) => {
        pub fn $name_width(mut self, width: $type) -> Self {
            self.layout.$name.width = width;
            self
        }

        pub fn $name_height(mut self, height: $type) -> Self {
            self.layout.$name.height = height;
            self
        }

        pub fn $name_all(mut self, all: $type) -> Self {
            self.layout.$name.width = all;
            self.layout.$name.height = all;
            self
        }
    };
}

macro_rules! layout_setter_line {
    (
        $name:ident : Line<$type:ty>
        as $name_start:ident, $name_end:ident
        as $name_all:ident
    ) => {
        pub fn $name_start(mut self, start: $type) -> Self {
            self.layout.$name.start = start;
            self
        }

        pub fn $name_end(mut self, end: $type) -> Self {
            self.layout.$name.end = end;
            self
        }

        pub fn $name_all(mut self, all: $type) -> Self {
            self.layout.$name.start = all;
            self.layout.$name.end = all;
            self
        }
    };
}

macro_rules! layout_setter_rect {
    (
        $name:ident : Rect<$type:ty>
        as $name_left:ident, $name_right:ident, $name_top:ident, $name_bottom:ident
        as $name_horiz:ident, $name_vert:ident, $name_all:ident
    ) => {
        pub fn $name_left(mut self, left: $type) -> Self {
            self.layout.$name.left = left;
            self
        }

        pub fn $name_right(mut self, right: $type) -> Self {
            self.layout.$name.right = right;
            self
        }

        pub fn $name_top(mut self, top: $type) -> Self {
            self.layout.$name.top = top;
            self
        }

        pub fn $name_bottom(mut self, left: $type) -> Self {
            self.layout.$name.bottom = left;
            self
        }

        pub fn $name_horiz(mut self, horizontal: $type) -> Self {
            self.layout.$name.left = horizontal;
            self.layout.$name.right = horizontal;
            self
        }

        pub fn $name_vert(mut self, vertical: $type) -> Self {
            self.layout.$name.top = vertical;
            self.layout.$name.bottom = vertical;
            self
        }

        pub fn $name_all(mut self, all: $type) -> Self {
            self.layout.$name.left = all;
            self.layout.$name.right = all;
            self.layout.$name.top = all;
            self.layout.$name.bottom = all;
            self
        }
    };
}

impl<C> Node<C> {
    layout_setter!(display: Display);

    layout_setter!(overflow: Point<Overflow>);
    layout_setter_point!(overflow: Point<Overflow>
        as overflow_x, overflow_y
        as overflow_all);

    layout_setter!(scrollbar_width: f32);
    layout_setter!(position: Position);

    layout_setter!(inset: Rect<LengthPercentageAuto>);
    layout_setter_rect!(inset: Rect<LengthPercentageAuto>
        as inset_left, inset_right, inset_top, inset_bottom
        as inset_horiz, inset_vert, inset_all);

    layout_setter!(size: Size<Dimension>);
    layout_setter_size!(size: Size<Dimension>
        as size_width, size_height
        as size_all);

    layout_setter!(min_size: Size<Dimension>);
    layout_setter_size!(min_size: Size<Dimension>
        as min_size_width, min_size_height
        as min_size_all);

    layout_setter!(max_size: Size<Dimension>);
    layout_setter_size!(max_size: Size<Dimension>
        as max_size_width, max_size_height
        as max_size_all);

    layout_setter!(aspect_ratio: Option<f32>);

    layout_setter!(margin: Rect<LengthPercentageAuto>);
    layout_setter_rect!(margin: Rect<LengthPercentageAuto>
        as margin_left, margin_right, margin_top, margin_bottom
        as margin_horiz, margin_vert, margin_all);

    layout_setter!(padding: Rect<LengthPercentage>);
    layout_setter_rect!(padding: Rect<LengthPercentage>
        as padding_left, padding_right, padding_top, padding_bottom
        as padding_horiz, padding_vert, padding_all);

    layout_setter!(border: Rect<LengthPercentage>);
    layout_setter_rect!(border: Rect<LengthPercentage>
        as border_left, border_right, border_top, border_bottom
        as border_horiz, border_vert, border_all);

    layout_setter!(align_items: Option<AlignItems>);
    layout_setter!(align_self: Option<AlignSelf>);
    layout_setter!(justify_items: Option<AlignItems>);
    layout_setter!(justify_self: Option<AlignSelf>);
    layout_setter!(align_content: Option<AlignContent>);
    layout_setter!(justify_content: Option<JustifyContent>);

    layout_setter!(gap: Size<LengthPercentage>);
    layout_setter_size!(gap: Size<LengthPercentage>
        as gap_width, gap_height
        as gap_all);

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

    layout_setter!(grid_row: Line<GridPlacement>);
    layout_setter_line!(grid_row: Line<GridPlacement>
        as grid_row_start, grid_row_end
        as grid_row_all);

    layout_setter!(grid_column: Line<GridPlacement>);
    layout_setter_line!(grid_column: Line<GridPlacement>
        as grid_column_start, grid_column_end
        as grid_column_all);
}
