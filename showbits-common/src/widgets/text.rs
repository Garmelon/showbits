use cosmic_text::{Attrs, Buffer, FontSystem, Metrics, Shaping, SwashCache};
use palette::Srgba;
use taffy::prelude::{AvailableSpace, Size};

use crate::{color, Rect, Vec2, View, Widget};

// https://github.com/DioxusLabs/taffy/blob/main/examples/cosmic_text.rs

pub struct FontStuff {
    font_system: FontSystem,
    swash_cache: SwashCache,
}

impl FontStuff {
    pub fn new() -> Self {
        Self {
            font_system: FontSystem::new(),
            swash_cache: SwashCache::new(),
        }
    }
}

impl Default for FontStuff {
    fn default() -> Self {
        Self::new()
    }
}

pub trait HasFontStuff {
    fn font_stuff(&mut self) -> &mut FontStuff;
}

pub struct Text {
    buffer: Buffer,
    color: Srgba,
}

impl Text {
    /// Default text color.
    const COLOR: Srgba = Srgba::new(0.0, 0.0, 0.0, 1.0);

    // Default shaping strategy.
    const SHAPING: Shaping = Shaping::Advanced;

    pub fn simple(
        font_stuff: &mut FontStuff,
        metrics: Metrics,
        attrs: Attrs<'_>,
        text: &str,
    ) -> Self {
        let fs = &mut font_stuff.font_system;

        let mut buffer = Buffer::new_empty(metrics);
        buffer.set_size(fs, f32::INFINITY, f32::INFINITY);
        buffer.set_text(fs, text, attrs, Self::SHAPING);

        Self {
            buffer,
            color: Self::COLOR,
        }
    }

    pub fn rich<'r, 's, I>(
        font_stuff: &mut FontStuff,
        metrics: Metrics,
        default_attrs: Attrs<'_>,
        spans: I,
    ) -> Self
    where
        I: IntoIterator<Item = (&'s str, Attrs<'r>)>,
    {
        let fs = &mut font_stuff.font_system;

        let mut buffer = Buffer::new_empty(metrics);
        buffer.set_size(fs, f32::INFINITY, f32::INFINITY);
        buffer.set_rich_text(fs, spans, default_attrs, Self::SHAPING);

        Self {
            buffer,
            color: Self::COLOR,
        }
    }

    pub fn color(mut self, color: Srgba) -> Self {
        self.color = color;
        self
    }
}

impl<C: HasFontStuff> Widget<C> for Text {
    fn size(
        &mut self,
        ctx: &mut C,
        known: Size<Option<f32>>,
        available: Size<AvailableSpace>,
    ) -> Size<f32> {
        let width = known.width.unwrap_or(match available.width {
            AvailableSpace::Definite(width) => width,
            AvailableSpace::MinContent => 0.0,
            AvailableSpace::MaxContent => f32::INFINITY,
        });

        let fs = &mut ctx.font_stuff().font_system;
        self.buffer.set_size(fs, width, f32::INFINITY);
        self.buffer.shape_until_scroll(fs, false);

        let runs = self.buffer.layout_runs();
        let height = runs.len() as f32 * self.buffer.metrics().line_height;
        let width = runs
            .map(|run| run.line_w)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0);

        // If we don't round up here, the layout rounding may round down our
        // size slightly. This may lead to more line breaks, moving some words
        // below our visible area.
        let width = width.ceil();
        let height = height.ceil();

        Size { width, height }
    }

    fn draw_below(&mut self, ctx: &mut C, view: &mut View<'_>) -> anyhow::Result<()> {
        let size = view.size();

        let FontStuff {
            font_system: fs,
            swash_cache: sc,
        } = ctx.font_stuff();

        self.buffer.set_size(fs, size.x as f32, size.y as f32);
        self.buffer.shape_until_scroll(fs, true);

        let color = color::to_text_color(self.color);
        self.buffer.draw(fs, sc, color, |x, y, w, h, color| {
            let color = color::from_text_color(color);
            let area = Rect::from_nw(Vec2::new(x, y), Vec2::from_u32(w, h));
            view.rect(area, color);
        });

        Ok(())
    }
}
