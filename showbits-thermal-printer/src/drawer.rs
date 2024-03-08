use tokio::sync::mpsc;

use crate::printer::Printer;

pub enum Command {
    Stop,
    Rip,
    Test,
    Text(String),
    ChatMessage { username: String, content: String },
}

pub struct Drawer {
    rx: mpsc::Receiver<Command>,
    printer: Printer,
}

impl Drawer {
    pub fn new(rx: mpsc::Receiver<Command>, printer: Printer) -> Self {
        Self { rx, printer }
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
            Command::Test => todo!(),
            Command::Text(_) => todo!(),
            Command::ChatMessage { username, content } => todo!(),
        }
        Ok(())
    }

    // fn on_rip(&mut self) -> anyhow::Result<()> {
    //     self.printer.init()?.feeds(6)?.print()?;
    //     Ok(())
    // }

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
