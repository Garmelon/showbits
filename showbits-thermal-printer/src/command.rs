pub enum Command {
    Stop,
    Test,
    Rip,
    Text(String),
    ChatMessage { username: String, content: String },
}
