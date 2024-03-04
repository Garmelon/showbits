use std::path::Path;

use escpos::{
    driver::FileDriver,
    printer::Printer as EPrinter,
    utils::{PageCode, Protocol, ESC},
};
use tokio::sync::mpsc;

use crate::{command::Command, util};

pub struct Printer {
    rx: mpsc::Receiver<Command>,
    printer: EPrinter<FileDriver>,
}

impl Printer {
    pub fn new(rx: mpsc::Receiver<Command>, path: &Path) -> anyhow::Result<Self> {
        let driver = FileDriver::open(path)?;

        // Experimentation has determined that the printer uses PC437 and the
        // page code can't be changed.
        // https://en.wikipedia.org/wiki/Code_page_437
        // https://www.epson-biz.com/modules/ref_charcode_en/index.php?content_id=10
        let printer = EPrinter::new(driver, Protocol::default(), Some(PageCode::PC437));

        Ok(Self { rx, printer })
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        while let Some(command) = self.rx.blocking_recv() {
            match command {
                Command::Stop => break,
                Command::Test => self.on_test()?,
                Command::Rip => self.on_rip()?,
                Command::Text(text) => self.on_text(text)?,
                Command::ChatMessage { username, content } => {
                    self.on_chat_message(username, content)?
                }
            }
        }
        Ok(())
    }

    fn on_test(&mut self) -> anyhow::Result<()> {
        self.printer.init()?;

        let x = 48; // bytes
        let y = 48; // dots

        let m = 0;
        let x_l = x as u8;
        let x_h = (x >> 8) as u8;
        let y_l = y as u8;
        let y_h = (y >> 8) as u8;
        let mut command = vec![0x1D, b'v', b'0', m, x_l, x_h, y_l, y_h];
        for y in 0..y {
            for x in 0..x {
                // command.push((x + y) as u8);
                command.push(0b0000_0011);
            }
        }
        self.printer.custom(&command)?;

        self.printer.print()?;

        Ok(())
    }

    fn on_rip(&mut self) -> anyhow::Result<()> {
        self.printer.init()?.feeds(6)?.print()?;
        Ok(())
    }

    fn on_text(&mut self, text: String) -> anyhow::Result<()> {
        let text = util::sanitize(&text);
        self.printer.init()?.write(&text)?.print()?;
        Ok(())
    }

    fn on_chat_message(&mut self, username: String, content: String) -> anyhow::Result<()> {
        let username = util::sanitize(&username);
        let content = util::sanitize(&content);

        let username = username
            .chars()
            .map(|c| if c.is_ascii_whitespace() { '_' } else { c })
            .take(16)
            .collect::<String>();

        let content = content.chars().take(300).collect::<String>();

        self.printer
            .init()?
            .reverse(true)?
            .write(&username)?
            .reverse(false)?
            .write(" ")?
            .writeln(&content)?
            .print()?;

        Ok(())
    }
}
