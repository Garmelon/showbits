use std::path::PathBuf;

use escpos::{
    driver::FileDriver,
    printer::Printer as EPrinter,
    utils::{PageCode, Protocol, GS},
};
use image::{Rgba, RgbaImage};
use showbits_common::{color, Tree};
use taffy::{AvailableSpace, NodeId, Size};

pub struct Printer {
    printer: Option<EPrinter<FileDriver>>,
    export_path: Option<PathBuf>,
}

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

    pub fn new(
        printer_path: Option<PathBuf>,
        export_path: Option<PathBuf>,
    ) -> anyhow::Result<Self> {
        let printer = if let Some(path) = printer_path {
            let driver = FileDriver::open(&path)?;
            let printer = EPrinter::new(driver, Protocol::default(), Some(Self::PAGE_CODE));
            Some(printer)
        } else {
            None
        };

        Ok(Self {
            printer,
            export_path,
        })
    }

    pub fn feed(&mut self) -> anyhow::Result<()> {
        if let Some(printer) = &mut self.printer {
            printer.init()?.feeds(3)?.print()?;
        }

        Ok(())
    }

    pub fn print_tree<C>(
        &mut self,
        tree: &mut Tree<C>,
        ctx: &mut C,
        root: NodeId,
    ) -> anyhow::Result<()> {
        let available = Size {
            width: AvailableSpace::Definite(Self::WIDTH as f32),
            // TODO Maybe MinContent? If not, why not?
            height: AvailableSpace::MaxContent,
        };

        let image = tree.render(ctx, root, available)?;

        if let Some(path) = &self.export_path {
            image.save(path)?;
        }

        if let Some(printer) = &mut self.printer {
            Self::print_image_to_printer(printer, &image)?;
        }

        Ok(())
    }

    /// Uses the obsolete `GS v 0` command to print an image.
    ///
    /// The image is printed in chunks because the command used has a maximum
    /// amount of data it can handle. In-between chunks, the paper is not moved,
    /// meaning that chunks connect to each other seamlessly.
    ///
    /// <https://download4.epson.biz/sec_pubs/pos/reference_en/escpos/gs_lv_0.html>
    fn print_image_to_printer(
        printer: &mut EPrinter<FileDriver>,
        image: &RgbaImage,
    ) -> anyhow::Result<()> {
        assert_eq!(Self::WIDTH % 8, 0);
        assert_eq!(image.width(), Self::WIDTH);

        printer.init()?;

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

            printer.custom(&command)?;
        }

        printer.print()?;
        Ok(())
    }

    fn get_horizontal_byte_starting_at(image: &RgbaImage, x: u32, y: u32) -> u8 {
        let p7 = Self::pixel_to_bit(*image.get_pixel(x, y));
        let p6 = Self::pixel_to_bit(*image.get_pixel(x + 1, y));
        let p5 = Self::pixel_to_bit(*image.get_pixel(x + 2, y));
        let p4 = Self::pixel_to_bit(*image.get_pixel(x + 3, y));
        let p3 = Self::pixel_to_bit(*image.get_pixel(x + 4, y));
        let p2 = Self::pixel_to_bit(*image.get_pixel(x + 5, y));
        let p1 = Self::pixel_to_bit(*image.get_pixel(x + 6, y));
        let p0 = Self::pixel_to_bit(*image.get_pixel(x + 7, y));

        let b7 = if p7 { 0b1000_0000 } else { 0 };
        let b6 = if p6 { 0b0100_0000 } else { 0 };
        let b5 = if p5 { 0b0010_0000 } else { 0 };
        let b4 = if p4 { 0b0001_0000 } else { 0 };
        let b3 = if p3 { 0b0000_1000 } else { 0 };
        let b2 = if p2 { 0b0000_0100 } else { 0 };
        let b1 = if p1 { 0b0000_0010 } else { 0 };
        let b0 = if p0 { 0b0000_0001 } else { 0 };

        b7 + b6 + b5 + b4 + b3 + b2 + b1 + b0
    }

    /// Convert pixel to bit, `true` is black and `false` is white.
    ///
    /// Instead of doing the physically accurate thing, I do what makes the most
    /// sense visually.
    fn pixel_to_bit(pixel: Rgba<u8>) -> bool {
        let color = color::from_image_color(pixel);
        let avg = (color.red + color.green + color.blue) / 3.0;
        avg < 0.5 // true == black
    }
}
