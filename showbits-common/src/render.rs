use image::RgbImage;
use taffy::{AvailableSpace, NodeId, Point, Size, TaffyResult, TaffyTree};

use crate::{BoxedWidget, Rect, Vec2, View};

fn point_to_vec2(point: Point<f32>) -> Vec2 {
    Vec2::new(point.x as i32, point.y as i32)
}

fn size_to_vec2(size: Size<f32>) -> Vec2 {
    Vec2::new(size.width as i32, size.height as i32)
}

fn layout<C>(
    tree: &mut TaffyTree<BoxedWidget<C>>,
    ctx: &mut C,
    root: NodeId,
    available: Size<AvailableSpace>,
) -> TaffyResult<()> {
    tree.enable_rounding(); // Just to make sure
    tree.compute_layout_with_measure(root, available, |known, available, _node, context| {
        if let Some(widget) = context {
            widget.size(ctx, known, available)
        } else {
            Size::ZERO
        }
    })
}

fn render_to_view<C>(
    tree: &mut TaffyTree<BoxedWidget<C>>,
    ctx: &mut C,
    node: NodeId,
    view: &mut View<'_>,
) -> anyhow::Result<()> {
    let layout = tree.layout(node)?;
    let area = Rect::from_nw(point_to_vec2(layout.location), size_to_vec2(layout.size));
    let mut view = view.dup().zoom(area);

    // First pass
    if let Some(widget) = tree.get_node_context_mut(node) {
        widget.draw_below(ctx, &mut view)?;
    }

    // Render children
    let mut children = vec![];
    for child in tree.children(node)? {
        let order = tree.layout(child)?.order;
        children.push((order, child));
    }
    children.sort_unstable_by_key(|(order, _)| *order);
    for (_, child) in children {
        render_to_view(tree, ctx, child, &mut view)?;
    }

    // Second pass
    if let Some(widget) = tree.get_node_context_mut(node) {
        widget.draw_above(ctx, &mut view)?;
    }

    Ok(())
}

pub fn render<C>(
    tree: &mut TaffyTree<BoxedWidget<C>>,
    ctx: &mut C,
    root: NodeId,
    available: Size<AvailableSpace>,
) -> anyhow::Result<RgbImage> {
    layout(tree, ctx, root, available)?;

    let layout = tree.layout(root)?;
    assert_eq!(layout.location.x, 0.0);
    assert_eq!(layout.location.y, 0.0);
    // TODO Check how taffy treats the border?

    let (width, height) = size_to_vec2(layout.size).to_u32();
    let mut image = RgbImage::new(width, height);
    render_to_view(tree, ctx, root, &mut View::new(&mut image))?;

    Ok(image)
}
