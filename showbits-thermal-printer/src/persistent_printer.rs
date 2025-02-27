use std::{fs, io::ErrorKind, path::PathBuf};

use anyhow::{Context, bail};
use image::RgbaImage;
use jiff::Timestamp;
use showbits_common::Tree;
use taffy::{AvailableSpace, NodeId, Size};

use crate::printer::Printer;

pub struct PersistentPrinter {
    printer_file: Option<PathBuf>,
    export_file: Option<PathBuf>,
    queue_dir: PathBuf,

    printer: Option<Printer>,
}

impl PersistentPrinter {
    pub fn new(
        printer_file: Option<PathBuf>,
        export_file: Option<PathBuf>,
        queue_dir: PathBuf,
    ) -> Self {
        Self {
            printer_file,
            export_file,
            queue_dir,
            printer: None,
        }
    }

    fn render_tree_to_image<C>(
        tree: &mut Tree<C>,
        ctx: &mut C,
        root: NodeId,
    ) -> anyhow::Result<RgbaImage> {
        let available = Size {
            width: AvailableSpace::Definite(Printer::WIDTH as f32),
            // TODO Maybe MinContent? If not, why not?
            height: AvailableSpace::MaxContent,
        };

        tree.render(ctx, root, available)
    }

    fn print_image(&mut self, image: &RgbaImage) -> anyhow::Result<()> {
        let Some(printer) = &mut self.printer else {
            bail!("no printer found");
        };
        printer.print_image(image)?;
        Ok(())
    }

    fn reconnect_printer(&mut self) -> anyhow::Result<()> {
        let printer = Printer::new(self.printer_file.clone(), self.export_file.clone())?;
        self.printer = Some(printer);
        Ok(())
    }

    fn print_image_robustly(&mut self, image: &RgbaImage) -> anyhow::Result<()> {
        println!("Printing image");
        if self.print_image(image).is_ok() {
            return Ok(());
        }
        println!("First attempt failed, reconnecting and retrying");
        self.reconnect_printer()?;
        self.print_image(image)?;
        Ok(())
    }

    fn enqueue_image(&mut self, image: &RgbaImage) -> anyhow::Result<()> {
        let now = Timestamp::now();
        let path = self.queue_dir.join(format!("{now}.png"));
        println!("Enqueuing image {}", path.display());

        fs::create_dir_all(&self.queue_dir)
            .with_context(|| format!("At {}", self.queue_dir.display()))
            .context("Failed to create queue directory")?;

        image
            .save(&path)
            .with_context(|| format!("At {}", path.display()))
            .context("Failed to save image to queue")?;

        Ok(())
    }

    pub fn print_tree<C>(
        &mut self,
        tree: &mut Tree<C>,
        ctx: &mut C,
        root: NodeId,
    ) -> anyhow::Result<()> {
        let image = Self::render_tree_to_image(tree, ctx, root)?;
        if self.print_image_robustly(&image).is_err() {
            self.enqueue_image(&image)?;
        }
        Ok(())
    }

    pub fn print_backlog(&mut self) -> anyhow::Result<()> {
        let mut files = vec![];

        match self.queue_dir.read_dir() {
            Err(err) if err.kind() == ErrorKind::NotFound => {}

            Err(err) => Err(err)
                .with_context(|| format!("At {}", self.queue_dir.display()))
                .context("Failed to open queue dir")?,

            Ok(dir) => {
                for entry in dir {
                    let entry = entry?;
                    if entry.file_type()?.is_file() {
                        files.push(entry.path());
                    }
                }
            }
        }

        files.sort_unstable();

        for file in files {
            println!("Dequeuing image {}", file.display());
            let image: RgbaImage = image::open(&file)?.into_rgba8();
            if self.print_image_robustly(&image).is_err() {
                return Ok(());
            }
            fs::remove_file(&file)?;
        }

        Ok(())
    }
}
