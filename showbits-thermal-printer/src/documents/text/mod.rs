use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};

use crate::{
    drawer::{Command, NewTypstDrawing},
    server::Server,
};

#[derive(Serialize)]
struct Data {
    text: String,
    force_wrap: bool,
    feed: bool,
}

#[derive(Deserialize)]
pub struct FormData {
    pub text: String,
    pub force_wrap: Option<bool>,
    pub feed: Option<bool>,
}

pub async fn post(server: State<Server>, Form(form): Form<FormData>) {
    let data = Data {
        text: form.text,
        force_wrap: form.force_wrap.unwrap_or(false),
        feed: form.feed.unwrap_or(true),
    };

    let typst = super::typst_with_lib()
        .with_json("/data.json", &data)
        .with_main_file(include_str!("main.typ"));

    let _ = server
        .tx
        .send(Command::draw(NewTypstDrawing::new(typst)))
        .await;
}
