use std::path::PathBuf;

use anyhow::Context;
use escpos::{
    driver::FileDriver,
    printer::Printer as EPrinter,
    printer_options::PrinterOptions,
    utils::{GS, PageCode, Protocol},
};
use image::RgbImage;

use crate::color::Color;

pub struct Printer(EPrinter<FileDriver>);

impl Printer {
    /// Experimentation has determined that the printer uses PC437 and the page
    /// code can't be changed.
    ///
    /// <https://en.wikipedia.org/wiki/Code_page_437>
    /// <https://download4.epson.biz/sec_pubs/pos/reference_en/charcode/page_00.html>
    const PAGE_CODE: PageCode = PageCode::PC437;

    /// Width of the printable area in pixels.
    ///
    /// Assumed to be a multiple of 8, then measured to that precision.
    pub const WIDTH: u32 = 8 * 48;

    /// Images are printed in chunks because a single print command can only
    /// print so much data.
    ///
    /// Looking at the [epson docs][0], most printers seem to support a max
    /// height of 2303, though some go up to 4095. Because I don't want to waste
    /// a bunch of paper trying various different heights, I'll go with 1023
    /// because it's nice and round and slightly conservative.
    ///
    /// [0]: https://download4.epson.biz/sec_pubs/pos/reference_en/escpos/gs_lv_0.html
    const CHUNK_HEIGHT: u32 = 0b0000_0011_1111_1111;

    pub fn new(path: PathBuf) -> anyhow::Result<Self> {
        let driver = FileDriver::open(&path)
            .with_context(|| format!("At {}", path.display()))
            .context("Failed to open printer driver")?;
        let protocol = Protocol::default();

        let mut options = PrinterOptions::default();
        options.page_code(Some(Self::PAGE_CODE));

        let printer = EPrinter::new(driver, protocol, Some(options));
        Ok(Self(printer))
    }

    /// Uses the obsolete `GS v 0` command to print an image.
    ///
    /// The image is printed in chunks because the command used has a maximum
    /// amount of data it can handle. In-between chunks, the paper is not moved,
    /// meaning that chunks connect to each other seamlessly.
    ///
    /// <https://download4.epson.biz/sec_pubs/pos/reference_en/escpos/gs_lv_0.html>
    pub fn print_image(&mut self, image: &RgbImage) -> anyhow::Result<()> {
        assert_eq!(Self::WIDTH % 8, 0);
        assert_eq!(image.width(), Self::WIDTH);

        self.0.init()?;

        for y_offset in (0..image.height()).step_by(Self::CHUNK_HEIGHT as usize) {
            // The command takes the width in bytes (groups of 8 pixels) and the
            // height in pixels. Both are then split into two bytes and sent.
            let chunk_width = Self::WIDTH / 8;
            let chunk_height = Self::CHUNK_HEIGHT.min(image.height() - y_offset);

            let m = 0; // Normal resolution
            let [_, _, x_h, x_l] = chunk_width.to_be_bytes();
            let [_, _, y_h, y_l] = chunk_height.to_be_bytes();
            let mut command = vec![GS, b'v', b'0', m, x_l, x_h, y_l, y_h];

            for y in y_offset..y_offset + chunk_height {
                for x in (0..Self::WIDTH).step_by(8) {
                    command.push(Self::get_horizontal_byte_starting_at(image, x, y));
                }
            }

            self.0.custom(&command)?;
        }

        self.0.print()?;
        Ok(())
    }

    fn get_horizontal_byte_starting_at(image: &RgbImage, x: u32, y: u32) -> u8 {
        let p7 = Color::from_rgb(*image.get_pixel(x, y));
        let p6 = Color::from_rgb(*image.get_pixel(x + 1, y));
        let p5 = Color::from_rgb(*image.get_pixel(x + 2, y));
        let p4 = Color::from_rgb(*image.get_pixel(x + 3, y));
        let p3 = Color::from_rgb(*image.get_pixel(x + 4, y));
        let p2 = Color::from_rgb(*image.get_pixel(x + 5, y));
        let p1 = Color::from_rgb(*image.get_pixel(x + 6, y));
        let p0 = Color::from_rgb(*image.get_pixel(x + 7, y));

        let b7 = if p7 == Color::Black { 0b1000_0000 } else { 0 };
        let b6 = if p6 == Color::Black { 0b0100_0000 } else { 0 };
        let b5 = if p5 == Color::Black { 0b0010_0000 } else { 0 };
        let b4 = if p4 == Color::Black { 0b0001_0000 } else { 0 };
        let b3 = if p3 == Color::Black { 0b0000_1000 } else { 0 };
        let b2 = if p2 == Color::Black { 0b0000_0100 } else { 0 };
        let b1 = if p1 == Color::Black { 0b0000_0010 } else { 0 };
        let b0 = if p0 == Color::Black { 0b0000_0001 } else { 0 };

        b7 + b6 + b5 + b4 + b3 + b2 + b1 + b0
    }
}
