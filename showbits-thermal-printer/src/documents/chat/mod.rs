use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};

use crate::server::{Server, somehow};

#[derive(Serialize)]
struct Data {
    username: String,
    content: String,
    feed: bool,
}

#[derive(Deserialize)]
pub struct FormData {
    pub username: String,
    pub content: String,
    pub feed: Option<bool>,
}

pub async fn post(server: State<Server>, Form(form): Form<FormData>) -> somehow::Result<()> {
    let data = Data {
        username: form.username,
        content: form.content,
        feed: form.feed.unwrap_or(false),
    };

    let typst = super::typst_with_lib()
        .with_json("/data.json", &data)
        .with_main_file(include_str!("main.typ"));

    server.print_typst(typst).await
}
