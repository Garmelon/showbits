use cosmic_text::{Attrs, Metrics};
use palette::Srgb;
use showbits_common::{
    widgets::{FontStuff, HasFontStuff, Text},
    Tree, WidgetExt,
};
use taffy::style_helpers::length;
use tokio::sync::mpsc;

use crate::printer::Printer;

pub enum Command {
    Stop,
    Rip,
    Test,
    Text(String),
    ChatMessage { username: String, content: String },
}

#[derive(Default)]
struct Context {
    font_stuff: FontStuff,
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
    pub fn new(rx: mpsc::Receiver<Command>, printer: Printer) -> Self {
        Self {
            rx,
            printer,
            ctx: Context::default(),
        }
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        while let Some(command) = self.rx.blocking_recv() {
            if matches!(command, Command::Stop) {
                break;
            };

            self.on_command(command)?;
        }
        Ok(())
    }

    fn on_command(&mut self, command: Command) -> anyhow::Result<()> {
        match command {
            Command::Stop => {} // Already handled one level above
            Command::Rip => self.printer.rip()?,
            Command::Test => self.on_test()?,
            Command::Text(_) => todo!(),
            Command::ChatMessage { username, content } => todo!(),
        }
        Ok(())
    }

    fn on_test(&mut self) -> anyhow::Result<()> {
        let mut tree = Tree::<Context>::new();

        let root = Text::simple(
            &mut self.ctx.font_stuff,
            Metrics::new(16.0, 32.0),
            Attrs::new(),
            "Hello world!",
        )
        .color(Srgb::new(1.0, 1.0, 1.0))
        .node()
        .margin_all(length(10.0))
        .register(&mut tree)?;

        self.printer.print_tree(&mut tree, &mut self.ctx, root)?;
        Ok(())
    }

    // fn on_text(&mut self, text: String) -> anyhow::Result<()> {
    //     let text = util::sanitize(&text);
    //     self.printer.init()?.write(&text)?.print()?;
    //     Ok(())
    // }

    // fn on_chat_message(&mut self, username: String, content: String) -> anyhow::Result<()> {
    //     let username = util::sanitize(&username);
    //     let content = util::sanitize(&content);

    //     let username = username
    //         .chars()
    //         .map(|c| if c.is_ascii_whitespace() { '_' } else { c })
    //         .take(16)
    //         .collect::<String>();

    //     let content = content.chars().take(300).collect::<String>();

    //     self.printer
    //         .init()?
    //         .reverse(true)?
    //         .write(&username)?
    //         .reverse(false)?
    //         .write(" ")?
    //         .writeln(&content)?
    //         .print()?;

    //     Ok(())
    // }
}
