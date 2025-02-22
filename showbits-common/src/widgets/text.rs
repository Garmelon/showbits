use cosmic_text::{Attrs, AttrsOwned, Buffer, Family, FontSystem, Metrics, Shaping, SwashCache};
use palette::Srgba;
use showbits_assets::{UNIFONT, UNIFONT_JP, UNIFONT_NAME, UNIFONT_SIZE, UNIFONT_UPPER};
use taffy::{
    Layout,
    prelude::{AvailableSpace, Size},
};

use crate::{Rect, Vec2, View, Widget, color};

// https://github.com/DioxusLabs/taffy/blob/main/examples/cosmic_text.rs

pub struct FontStuff {
    font_system: FontSystem,
    swash_cache: SwashCache,
}

impl FontStuff {
    pub fn new() -> Self {
        let mut font_system = FontSystem::new();
        let db = font_system.db_mut();
        db.load_font_data(UNIFONT.to_vec());
        db.load_font_data(UNIFONT_JP.to_vec());
        db.load_font_data(UNIFONT_UPPER.to_vec());
        db.set_monospace_family(UNIFONT_NAME);

        Self {
            font_system,
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
    metrics: Metrics,
    default_attrs: AttrsOwned,
    chunks: Vec<(AttrsOwned, String)>,
    shaping: Shaping,
    color: Srgba,
}

impl Text {
    pub const fn default_metrics() -> Metrics {
        Metrics::new(UNIFONT_SIZE, UNIFONT_SIZE)
    }

    pub fn default_attrs<'a>() -> Attrs<'a> {
        Attrs::new().family(Family::Monospace)
    }

    pub fn new() -> Self {
        Self {
            metrics: Self::default_metrics(),
            default_attrs: AttrsOwned::new(Self::default_attrs()),
            chunks: vec![],
            shaping: Shaping::Advanced,
            color: color::BLACK,
        }
    }

    pub fn with_metrics(mut self, metrics: Metrics) -> Self {
        self.metrics = metrics;
        self
    }

    pub fn with_font_size(mut self, size: f32) -> Self {
        self.metrics.font_size = size;
        self
    }

    pub fn with_line_height(mut self, height: f32) -> Self {
        self.metrics.line_height = height;
        self
    }

    pub fn with_default_attrs(mut self, attrs: Attrs<'_>) -> Self {
        self.default_attrs = AttrsOwned::new(attrs);
        self
    }

    pub fn with_shaping(mut self, shaping: Shaping) -> Self {
        self.shaping = shaping;
        self
    }

    pub fn with_color(mut self, color: Srgba) -> Self {
        self.color = color;
        self
    }

    pub fn and_plain<S: ToString>(mut self, text: S) -> Self {
        let chunk = (self.default_attrs.clone(), text.to_string());
        self.chunks.push(chunk);
        self
    }

    pub fn and_rich<S: ToString>(mut self, attrs: Attrs<'_>, text: S) -> Self {
        let chunk = (AttrsOwned::new(attrs), text.to_string());
        self.chunks.push(chunk);
        self
    }

    pub fn and_chunks<I>(mut self, chunks: I) -> Self
    where
        I: IntoIterator<Item = (AttrsOwned, String)>,
    {
        self.chunks.extend(chunks);
        self
    }

    pub fn widget<C: HasFontStuff>(self, font_stuff: &mut FontStuff) -> impl Widget<C> + use<C> {
        let fs = &mut font_stuff.font_system;
        let mut buffer = Buffer::new_empty(self.metrics);
        buffer.set_size(fs, f32::INFINITY, f32::INFINITY);

        let spans = self
            .chunks
            .iter()
            .map(|(attrs, text)| (text as &str, attrs.as_attrs()));
        buffer.set_rich_text(fs, spans, self.default_attrs.as_attrs(), self.shaping);

        TextWidget {
            buffer,
            color: self.color,
        }
    }
}

impl Default for Text {
    fn default() -> Self {
        Self::new()
    }
}

struct TextWidget {
    buffer: Buffer,
    color: Srgba,
}

impl<C: HasFontStuff> Widget<C> for TextWidget {
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

    fn draw_below(
        &mut self,
        ctx: &mut C,
        view: &mut View<'_>,
        _layout: &Layout,
    ) -> anyhow::Result<()> {
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
