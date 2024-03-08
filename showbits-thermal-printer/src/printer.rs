use std::path::{Path, PathBuf};

use escpos::{
    driver::FileDriver,
    printer::Printer as EPrinter,
    utils::{PageCode, Protocol},
};

pub struct Printer {
    printer: Option<EPrinter<FileDriver>>,
    export_path: Option<PathBuf>,
}

impl Printer {
    /// Experimentation has determined that the printer uses PC437 and the page
    /// code can't be changed.
    ///
    /// https://en.wikipedia.org/wiki/Code_page_437
    /// https://www.epson-biz.com/modules/ref_charcode_en/index.php?content_id=10
    const PAGE_CODE: PageCode = PageCode::PC437;

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

    pub fn rip(&mut self) -> anyhow::Result<()> {
        if let Some(printer) = &mut self.printer {
            printer.init()?.feeds(4)?.print()?;
        }

        Ok(())
    }
}
