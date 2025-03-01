use std::io::Cursor;

use anyhow::Context;
use image::{ImageFormat, RgbaImage};
use showbits_typst::Typst;

pub struct Image {
    pub image: RgbaImage,
}

impl Image {
    pub fn into_typst(self) -> anyhow::Result<Typst> {
        let mut bytes: Vec<u8> = Vec::new();
        self.image
            .write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)
            .context("failed to encode image as png")?;

        let typst = super::typst_with_lib()
            .with_file("/image.png", bytes)
            .with_main_file(include_str!("main.typ"));

        Ok(typst)
    }
}
