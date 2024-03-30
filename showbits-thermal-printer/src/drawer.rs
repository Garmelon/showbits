mod calendar;
mod cells;
mod image;
mod text;

use ::image::{Luma, Pixel, RgbaImage};
use showbits_common::{
    color::{BLACK, WHITE},
    widgets::{Block, FontStuff, HasFontStuff, Image, Text},
    Node, Tree, WidgetExt,
};
use taffy::{
    style_helpers::{length, percent},
    AlignItems, Display, FlexDirection,
};
use tokio::sync::mpsc;

use crate::printer::Printer;

pub use self::{
    calendar::CalendarDrawing, cells::CellsDrawing, image::ImageDrawing, text::TextDrawing,
};

#[derive(Default)]
pub struct Context {
    font_stuff: FontStuff,
}

pub trait Drawing {
    fn draw(&self, printer: &mut Printer, ctx: &mut Context) -> anyhow::Result<()>;
}

pub struct BoxedDrawing(Box<dyn Drawing + Send>);

pub enum Command {
    Draw(BoxedDrawing),

    Photo { image: RgbaImage, title: String },
    ChatMessage { username: String, content: String },
}

impl Command {
    pub fn draw<D: Drawing + Send + 'static>(drawing: D) -> Self {
        Self::Draw(BoxedDrawing(Box::new(drawing)))
    }
}

impl HasFontStuff for Context {
    fn font_stuff(&mut self) -> &mut FontStuff {
        &mut self.font_stuff
    }
}

pub struct Drawer {
    rx: mpsc::Receiver<Command>,
    printer: Printer,
    ctx: Context,
}

impl Drawer {
    const FEED: f32 = 64.0;

    pub fn new(rx: mpsc::Receiver<Command>, printer: Printer) -> Self {
        Self {
            rx,
            printer,
            ctx: Context::default(),
        }
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        while let Some(command) = self.rx.blocking_recv() {
            self.on_command(command)?;
        }
        Ok(())
    }

    fn on_command(&mut self, command: Command) -> anyhow::Result<()> {
        match command {
            Command::Draw(drawing) => drawing.0.draw(&mut self.printer, &mut self.ctx)?,

            Command::Photo { image, title } => self.on_photo(image, title)?,
            Command::ChatMessage { username, content } => {
                self.on_chat_message(username, content)?
            }
        }
        Ok(())
    }

    fn on_photo(&mut self, mut image: RgbaImage, title: String) -> anyhow::Result<()> {
        println!(
            "Printing photo {title:?} ({}x{})",
            image.width(),
            image.height()
        );
        let mut tree = Tree::<Context>::new(WHITE);

        for pixel in image.pixels_mut() {
            let [l] = pixel.to_luma().0;
            let l = l as f32 / 255.0; // Convert to [0, 1]
            let l = 1.0 - (0.4 * (1.0 - l)); // Lerp to [0.6, 1]
            let l = (l.clamp(0.0, 1.0) * 255.0) as u8; // Convert back to [0, 255]
            *pixel = Luma([l]).to_rgba();
        }

        let image = Image::new(image)
            .with_dither_palette(&[BLACK, WHITE])
            .node()
            .register(&mut tree)?;

        let title = Text::new()
            .with_metrics(Text::default_metrics().scale(2.0))
            .and_plain(title)
            .widget(&mut self.ctx.font_stuff)
            .node()
            .register(&mut tree)?;

        let root = Node::empty()
            .with_size_width(percent(1.0))
            .with_padding_bottom(length(Self::FEED))
            .with_display(Display::Flex)
            .with_flex_direction(FlexDirection::Column)
            .with_align_items(Some(AlignItems::Center))
            .with_gap(length(8.0))
            .and_child(image)
            .and_child(title)
            .register(&mut tree)?;

        self.printer.print_tree(&mut tree, &mut self.ctx, root)?;
        Ok(())
    }

    fn on_chat_message(&mut self, username: String, content: String) -> anyhow::Result<()> {
        let mut tree = Tree::<Context>::new(WHITE);

        let max_username_width_in_chars = 32.0;
        let max_username_height_in_lines = 3.0;
        let max_content_height_in_lines = 16.0;

        let username = Text::new()
            .and_plain(username)
            .widget(&mut self.ctx.font_stuff)
            .node()
            .with_max_size_width(length(max_username_width_in_chars * 8.0))
            .with_max_size_height(length(max_username_height_in_lines * 16.0))
            .register(&mut tree)?;

        let username = Block::new()
            .with_border(BLACK)
            .node()
            .with_border_all(length(1.0))
            .with_padding_horiz(length(1.0))
            .with_flex_shrink(0.0) // Avoid wrapping
            .and_child(username)
            .register(&mut tree)?;

        let content = if let Some(content) = content.strip_prefix("/me") {
            let content = content.trim_start();

            let content = Text::new()
                .and_plain(content)
                .widget(&mut self.ctx.font_stuff)
                .node()
                .with_max_size_height(length(max_content_height_in_lines * 16.0))
                .register(&mut tree)?;

            Block::new()
                .with_border(BLACK)
                .node()
                .with_border_all(length(1.0))
                .with_padding_horiz(length(1.0))
                .and_child(content)
                .register(&mut tree)?
        } else {
            let content = Text::new()
                .and_plain(content)
                .widget(&mut self.ctx.font_stuff)
                .node()
                .with_max_size_height(length(max_content_height_in_lines * 16.0))
                .register(&mut tree)?;

            Node::empty()
                .with_padding_vert(length(1.0))
                .and_child(content)
                .register(&mut tree)?
        };

        let root = Node::empty()
            .with_size_width(percent(1.0))
            .with_padding_all(length(1.0))
            .with_flex_direction(FlexDirection::Row)
            .with_align_items(Some(AlignItems::Start))
            .with_gap_width(length(2.0))
            .and_child(username)
            .and_child(content)
            .register(&mut tree)?;

        self.printer.print_tree(&mut tree, &mut self.ctx, root)?;
        Ok(())
    }
}
