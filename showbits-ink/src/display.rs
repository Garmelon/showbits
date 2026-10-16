//! Controller for a WaveShare 7.5" HD (B) e-Paper display.
//!
//! This implementation is based on WaveShare's [python driver][3]. It follows
//! its structure relatively closely.
//!
//! Some of the values in the original python driver don't seem to match what's
//! on the [data sheet][2], but when I changed them to match the data sheet, the
//! image stopped being displayed correctly. As far as I'm concerned, all of the
//! commands and data (besides the pixel data itself) are magic numbers.
//!
//! I tried to send the display data in chunks of more than one byte, but the
//! display seemed to ignore my commands entirely, so I switched back to the
//! one-byte-at-a-time version from the original driver.
//!
//! # Links
//! - [Wiki page][1]
//! - [Data sheet][2]
//! - [Python driver][3]
//!
//! [1]: https://www.waveshare.com/wiki/7.5inch_HD_e-Paper_HAT_(B)
//! [2]: https://www.waveshare.com/w/upload/9/94/7.5inch_HD_e-Paper_B_Datasheet.pdf
//! [3]: https://github.com/waveshare/e-Paper/blob/551b05e2f7423ec5d0974dffe0f1b63e684e02a2/RaspberryPi_JetsonNano/python/lib/waveshare_epd/epd7in5b_HD.py

use std::thread;
use std::time::Duration;

use image::{Rgb, RgbImage};
use rppal::gpio::{Gpio, InputPin, OutputPin};
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};

pub const WIDTH: u32 = 880;
pub const HEIGHT: u32 = 528;

pub const WHITE: Rgb<u8> = Rgb([255, 255, 255]);
pub const BLACK: Rgb<u8> = Rgb([0, 0, 0]);
pub const RED: Rgb<u8> = Rgb([255, 0, 0]);

struct Eps7in5bHd {
    pin_dc: OutputPin,
    pin_rst: OutputPin,
    pin_busy: InputPin,
    spi: Spi,
}

impl Eps7in5bHd {
    fn new() -> anyhow::Result<Self> {
        let gpio = Gpio::new()?;
        Ok(Self {
            pin_dc: gpio.get(25)?.into_output(),
            pin_rst: gpio.get(17)?.into_output(),
            pin_busy: gpio.get(24)?.into_input(),
            spi: Spi::new(Bus::Spi0, SlaveSelect::Ss0, 4000000, Mode::Mode0)?,
        })
    }

    fn reset(&mut self) {
        self.pin_rst.set_high();
        thread::sleep(Duration::from_millis(200));

        self.pin_rst.set_low();
        thread::sleep(Duration::from_millis(4));

        self.pin_rst.set_high();
        thread::sleep(Duration::from_millis(200));
    }

    fn send_command(&mut self, command: u8) -> anyhow::Result<()> {
        self.pin_dc.set_low();
        self.spi.write(&[command])?;

        Ok(())
    }

    fn send_data(&mut self, data: u8) -> anyhow::Result<()> {
        self.pin_dc.set_high();
        self.spi.write(&[data])?;

        Ok(())
    }

    fn block_while_busy(&mut self) {
        while self.pin_busy.is_high() {
            thread::sleep(Duration::from_millis(200))
        }
    }

    fn init(&mut self) -> anyhow::Result<()> {
        self.reset();

        self.send_command(0x12)?;
        self.block_while_busy();

        self.send_command(0x46)?;
        self.send_data(0xF7)?;
        self.block_while_busy();

        self.send_command(0x47)?;
        self.send_data(0xF7)?;
        self.block_while_busy();

        self.send_command(0x0C)?;
        self.send_data(0xAE)?;
        self.send_data(0xC7)?;
        self.send_data(0xC3)?;
        self.send_data(0xC0)?;
        self.send_data(0x40)?;

        self.send_command(0x01)?;
        self.send_data(0xAF)?;
        self.send_data(0x02)?;
        self.send_data(0x01)?;

        self.send_command(0x11)?;
        self.send_data(0x01)?;

        self.send_command(0x44)?;
        self.send_data(0x00)?;
        self.send_data(0x00)?;
        self.send_data(0x6F)?;
        self.send_data(0x03)?;
        self.send_command(0x45)?;
        self.send_data(0xAF)?;
        self.send_data(0x02)?;
        self.send_data(0x00)?;
        self.send_data(0x00)?;

        self.send_command(0x3C)?;
        self.send_data(0x01)?;

        self.send_command(0x18)?;
        self.send_data(0x80)?;
        self.send_command(0x22)?;
        self.send_data(0xB1)?;
        self.send_command(0x20)?;
        self.block_while_busy();

        self.send_command(0x4E)?;
        self.send_data(0x00)?;
        self.send_data(0x00)?;
        self.send_command(0x4F)?;
        self.send_data(0xAF)?;
        self.send_data(0x02)?;

        Ok(())
    }

    fn get_buffers(image: &RgbImage) -> (Vec<u8>, Vec<u8>) {
        let mut black_buffer = vec![0; (WIDTH * HEIGHT / 8) as usize];
        let mut red_buffer = vec![0; (WIDTH * HEIGHT / 8) as usize];

        for y in 0..HEIGHT {
            for i in 0..WIDTH / 8 {
                let x = i * 8;

                let p0 = image.get_pixel(x, y);
                let p1 = image.get_pixel(x + 1, y);
                let p2 = image.get_pixel(x + 2, y);
                let p3 = image.get_pixel(x + 3, y);
                let p4 = image.get_pixel(x + 4, y);
                let p5 = image.get_pixel(x + 5, y);
                let p6 = image.get_pixel(x + 6, y);
                let p7 = image.get_pixel(x + 7, y);

                // 0 if black, 1 otherwise
                let p0b = if *p0 != BLACK { 0b1000_0000 } else { 0 };
                let p1b = if *p1 != BLACK { 0b0100_0000 } else { 0 };
                let p2b = if *p2 != BLACK { 0b0010_0000 } else { 0 };
                let p3b = if *p3 != BLACK { 0b0001_0000 } else { 0 };
                let p4b = if *p4 != BLACK { 0b0000_1000 } else { 0 };
                let p5b = if *p5 != BLACK { 0b0000_0100 } else { 0 };
                let p6b = if *p6 != BLACK { 0b0000_0010 } else { 0 };
                let p7b = if *p7 != BLACK { 0b0000_0001 } else { 0 };
                let black = p0b | p1b | p2b | p3b | p4b | p5b | p6b | p7b;
                black_buffer[(y * WIDTH / 8 + i) as usize] = black;

                // 1 if red, 0 otherwise (contrary to what the wiki article says)
                let p0r = if *p0 == RED { 0b1000_0000 } else { 0 };
                let p1r = if *p1 == RED { 0b0100_0000 } else { 0 };
                let p2r = if *p2 == RED { 0b0010_0000 } else { 0 };
                let p3r = if *p3 == RED { 0b0001_0000 } else { 0 };
                let p4r = if *p4 == RED { 0b0000_1000 } else { 0 };
                let p5r = if *p5 == RED { 0b0000_0100 } else { 0 };
                let p6r = if *p6 == RED { 0b0000_0010 } else { 0 };
                let p7r = if *p7 == RED { 0b0000_0001 } else { 0 };
                let red = p0r | p1r | p2r | p3r | p4r | p5r | p6r | p7r;
                red_buffer[(y * WIDTH / 8 + i) as usize] = red;
            }
        }

        (black_buffer, red_buffer)
    }

    fn display(&mut self, black: &[u8], red: &[u8]) -> anyhow::Result<()> {
        self.send_command(0x4F)?;
        self.send_data(0xAF)?;

        self.send_command(0x24)?;
        for byte in black {
            self.send_data(*byte)?;
        }

        self.send_command(0x26)?;
        for byte in red {
            self.send_data(*byte)?;
        }

        self.send_command(0x22)?;
        self.send_data(0xC7)?;
        self.send_command(0x20)?;
        thread::sleep(Duration::from_millis(200));
        self.block_while_busy();

        Ok(())
    }

    fn sleep(mut self) -> anyhow::Result<()> {
        self.send_command(0x10)?;
        self.send_data(0x01)?;

        thread::sleep(Duration::from_secs(2));

        self.pin_rst.set_low();
        self.pin_dc.set_low();

        Ok(())
    }
}

pub fn display(image: &RgbImage) -> anyhow::Result<()> {
    assert_eq!(image.width(), WIDTH);
    assert_eq!(image.height(), HEIGHT);

    let (black, red) = Eps7in5bHd::get_buffers(image);
    let mut eps = Eps7in5bHd::new()?;

    eps.init()?;
    eps.display(&black, &red)?;
    eps.sleep()?;

    Ok(())
}
