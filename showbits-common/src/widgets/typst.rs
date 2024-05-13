use std::{fs, path::PathBuf, sync::OnceLock};

use anyhow::anyhow;
use comemo::Prehashed;
use image::RgbaImage;
use taffy::{
    prelude::{AvailableSpace, Size},
    Layout,
};
use typst::{
    diag::{FileError, FileResult},
    eval::Tracer,
    foundations::{Bytes, Datetime},
    layout::Abs,
    syntax::{FileId, Source},
    text::{Font, FontBook},
    visualize::Color,
    Library, World,
};

use crate::{View, Widget};

// The logic for detecting and loading fonts was ripped straight from:
// https://github.com/typst/typst/blob/69dcc89d84176838c293b2d59747cd65e28843ad/crates/typst-cli/src/fonts.rs
// https://github.com/typst/typst/blob/69dcc89d84176838c293b2d59747cd65e28843ad/crates/typst-cli/src/world.rs#L193-L195

struct FontSlot {
    path: PathBuf,
    index: u32,
    font: OnceLock<Option<Font>>,
}

impl FontSlot {
    pub fn get(&self) -> Option<Font> {
        self.font
            .get_or_init(|| {
                let data = fs::read(&self.path).ok()?.into();
                Font::new(data, self.index)
            })
            .clone()
    }
}

struct FontLoader {
    book: FontBook,
    fonts: Vec<FontSlot>,
}

impl FontLoader {
    fn new() -> Self {
        Self {
            book: FontBook::new(),
            fonts: vec![],
        }
    }

    fn load_embedded_fonts(&mut self) {
        // https://github.com/typst/typst/blob/be12762d942e978ddf2e0ac5c34125264ab483b7/crates/typst-cli/src/fonts.rs#L107-L121
        for font_file in typst_assets::fonts() {
            let font_data = Bytes::from_static(font_file);
            for (i, font) in Font::iter(font_data).enumerate() {
                self.book.push(font.info().clone());
                self.fonts.push(FontSlot {
                    path: PathBuf::new(),
                    index: i as u32,
                    font: OnceLock::from(Some(font)),
                });
            }
        }
    }
}

struct DummyWorld {
    library: Prehashed<Library>,
    book: Prehashed<FontBook>,
    main: Source,
    fonts: Vec<FontSlot>,
}

impl DummyWorld {
    fn new(main: String) -> Self {
        let mut loader = FontLoader::new();
        loader.load_embedded_fonts();

        Self {
            library: Prehashed::new(Library::builder().build()),
            book: Prehashed::new(loader.book),
            main: Source::detached(main),
            fonts: loader.fonts,
        }
    }
}

impl World for DummyWorld {
    fn library(&self) -> &Prehashed<Library> {
        &self.library
    }

    fn book(&self) -> &Prehashed<FontBook> {
        &self.book
    }

    fn main(&self) -> Source {
        self.main.clone()
    }

    fn source(&self, _id: FileId) -> FileResult<Source> {
        Err(FileError::AccessDenied)
    }

    fn file(&self, _id: FileId) -> FileResult<Bytes> {
        Err(FileError::AccessDenied)
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.fonts[index].get()
    }

    fn today(&self, _offset: Option<i64>) -> Option<Datetime> {
        None
    }
}

const SCALE: f32 = 3.0;

pub struct Typst {
    code: String,
}

impl Typst {
    pub fn new(code: String) -> Self {
        Self { code }
    }

    fn render(&self, width: Option<f32>, height: Option<f32>) -> Result<RgbaImage, Vec<String>> {
        let width = match width {
            Some(width) => format!("{}pt", width / SCALE),
            None => "auto".to_string(),
        };

        let height = match height {
            Some(height) => format!("{}pt", height / SCALE),
            None => "auto".to_string(),
        };

        let mut source = String::new();
        source.push_str(&format!("#set page(width: {width}, height: {height})\n"));
        source.push_str("#set page(margin: (left: 0mm, right: 0mm, top: 1mm, bottom: 2mm))\n");
        source.push_str(&self.code);

        let world = DummyWorld::new(source);
        let mut tracer = Tracer::new();

        let document = typst::compile(&world, &mut tracer).map_err(|errs| {
            errs.into_iter()
                .map(|sd| sd.message.to_string())
                .collect::<Vec<_>>()
        })?;

        let pixmap =
            typst_render::render_merged(&document, SCALE, Color::WHITE, Abs::zero(), Color::WHITE);

        let buffer = RgbaImage::from_raw(pixmap.width(), pixmap.height(), pixmap.take()).unwrap();

        Ok(buffer)
    }
}

impl<C> Widget<C> for Typst {
    fn size(
        &mut self,
        _ctx: &mut C,
        known: Size<Option<f32>>,
        available: Size<AvailableSpace>,
    ) -> Size<f32> {
        let width = known.width.or(match available.width {
            AvailableSpace::Definite(width) => Some(width),
            AvailableSpace::MinContent => None, // auto
            AvailableSpace::MaxContent => Some(4096.0),
        });

        let height = known.height.or(match available.height {
            AvailableSpace::Definite(width) => Some(width),
            AvailableSpace::MinContent | AvailableSpace::MaxContent => None, // auto
        });

        let Ok(buffer) = self.render(width, height) else {
            return Size {
                width: width.unwrap_or(0.0),
                height: height.unwrap_or(0.0),
            };
        };

        // Round up so we definitely have enough space.
        // let width = (buffer.width() as f32 / SCALE).ceil() * SCALE + 1.0;
        // let height = (buffer.height() as f32 / SCALE).ceil() * SCALE + 1.0;
        let width = (buffer.width() as f32).ceil() + 1.0;
        let height = (buffer.height() as f32).ceil() + 1.0;

        Size { width, height }
    }

    fn draw_below(
        &mut self,
        _ctx: &mut C,
        view: &mut View<'_>,
        _layout: &Layout,
    ) -> anyhow::Result<()> {
        let size = view.size();

        let buffer = self
            .render(Some(size.x as f32), Some(size.y as f32))
            .map_err(|errs| anyhow!("{}", errs.join("\n")))?;

        view.image(&buffer);

        Ok(())
    }
}
