use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    sync::OnceLock,
};

use anyhow::anyhow;
use image::RgbaImage;
use serde::Serialize;
use typst::{
    Library, World,
    diag::{FileError, FileResult},
    foundations::{Bytes, Datetime},
    layout::{Abs, PagedDocument},
    syntax::{FileId, Source, VirtualPath},
    text::{Font, FontBook},
    utils::LazyHash,
    visualize::Color,
};

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
                let data = fs::read(&self.path).ok()?;
                Font::new(Bytes::new(data), self.index)
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

    fn load_font_file(&mut self, data: &'static [u8]) {
        // https://github.com/typst/typst/blob/be12762d942e978ddf2e0ac5c34125264ab483b7/crates/typst-cli/src/fonts.rs#L107-L121
        let font_data = Bytes::new(data);
        for (i, font) in Font::iter(font_data).enumerate() {
            self.book.push(font.info().clone());
            self.fonts.push(FontSlot {
                path: PathBuf::new(),
                index: i as u32,
                font: OnceLock::from(Some(font)),
            });
        }
    }

    fn load_embedded_fonts(&mut self) {
        // https://github.com/typst/typst/blob/be12762d942e978ddf2e0ac5c34125264ab483b7/crates/typst-cli/src/fonts.rs#L107-L121
        for font_file in typst_assets::fonts() {
            self.load_font_file(font_file);
        }
    }

    fn load_unifonts(&mut self) {
        self.load_font_file(showbits_assets::UNIFONT);
        self.load_font_file(showbits_assets::UNIFONT_JP);
        self.load_font_file(showbits_assets::UNIFONT_UPPER);
    }
}

pub struct Typst {
    library: LazyHash<Library>,
    book: LazyHash<FontBook>,
    fonts: Vec<FontSlot>,
    files: HashMap<String, Vec<u8>>,
}

impl Typst {
    const MAIN_PATH: &str = "/main.typ";

    pub fn new() -> Self {
        let mut loader = FontLoader::new();
        loader.load_embedded_fonts();
        loader.load_unifonts();

        Self {
            library: LazyHash::new(Library::default()),
            book: LazyHash::new(loader.book),
            fonts: loader.fonts,
            files: HashMap::new(),
        }
    }

    pub fn add_file(&mut self, path: impl ToString, data: impl Into<Vec<u8>>) {
        let path = path.to_string();
        let data = data.into();
        self.files.insert(path, data);
    }

    pub fn with_file(mut self, path: impl ToString, data: impl Into<Vec<u8>>) -> Self {
        self.add_file(path, data);
        self
    }

    pub fn add_json<T: Serialize>(&mut self, path: impl ToString, data: &T) {
        let data = serde_json::to_vec(data).expect("data should serialize to json");
        self.add_file(path, data);
    }

    pub fn with_json<T: Serialize>(mut self, path: impl ToString, data: &T) -> Self {
        self.add_json(path, data);
        self
    }

    pub fn add_main_file(&mut self, data: impl Into<Vec<u8>>) {
        self.add_file(Self::MAIN_PATH, data);
    }

    pub fn with_main_file(mut self, data: impl Into<Vec<u8>>) -> Self {
        self.add_main_file(data);
        self
    }

    pub fn render(&self) -> anyhow::Result<RgbaImage> {
        let document = typst::compile::<PagedDocument>(self)
            .output
            .map_err(|err| {
                let msg = err
                    .into_iter()
                    .map(|it| it.message.to_string())
                    .collect::<Vec<_>>()
                    .join("\n");
                anyhow!("{msg}")
            })?;

        let pixmap = typst_render::render_merged(&document, 1.0, Abs::zero(), Some(Color::WHITE));

        RgbaImage::from_raw(pixmap.width(), pixmap.height(), pixmap.take())
            .ok_or(anyhow!("Failed to create image from raw pixel data"))
    }

    fn get_file_bytes(&self, path: &Path) -> FileResult<&[u8]> {
        let path_str = path
            .to_str()
            .ok_or_else(|| FileError::NotFound(path.to_path_buf()))?;

        let bytes = self
            .files
            .get(path_str)
            .ok_or_else(|| FileError::NotFound(path.to_path_buf()))?;

        Ok(bytes)
    }
}

impl Default for Typst {
    fn default() -> Self {
        Self::new()
    }
}

impl World for Typst {
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &self.book
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.fonts.get(index)?.get()
    }

    fn main(&self) -> FileId {
        FileId::new(None, VirtualPath::new(Self::MAIN_PATH))
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        // TODO Remove debug logging
        println!("Accessing source {id:?}");

        // TODO Do we need to handle packages ourselves?
        if id.package().is_some() {
            Err(FileError::AccessDenied)?
        }

        let path = id.vpath().as_rooted_path();
        let bytes = self.get_file_bytes(path)?.to_vec();
        let text = String::from_utf8(bytes).map_err(|_| FileError::InvalidUtf8)?;
        Ok(Source::new(id, text))
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        // TODO Remove debug logging
        println!("Accessing file {id:?}");

        // TODO Do we need to handle packages ourselves?
        if id.package().is_some() {
            Err(FileError::AccessDenied)?
        }

        let path = id.vpath().as_rooted_path();
        let bytes = self.get_file_bytes(path)?.to_vec();
        Ok(Bytes::new(bytes))
    }

    fn today(&self, _offset: Option<i64>) -> Option<Datetime> {
        None
    }
}
