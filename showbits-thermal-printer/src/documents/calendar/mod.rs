use axum::{Form, extract::State};
use jiff::Zoned;
use serde::{Deserialize, Serialize};

use crate::{
    drawer::{Command, NewTypstDrawing},
    server::{Server, somehow},
};

#[derive(Serialize)]
struct Data {
    year: i16,
    month: i8,
    feed: bool,
}

#[derive(Deserialize)]
pub struct FormData {
    pub year: Option<i16>,
    pub month: Option<i8>,
    pub feed: Option<bool>,
}

pub async fn post(server: State<Server>, Form(form): Form<FormData>) -> somehow::Result<()> {
    let date = Zoned::now().date();

    let data = Data {
        year: form.year.unwrap_or(date.year()),
        month: form.month.unwrap_or(date.month()),
        feed: form.feed.unwrap_or(true),
    };

    let typst = super::typst_with_lib()
        .with_json("/data.json", &data)
        .with_main_file(include_str!("main.typ"));

    let _ = server
        .tx
        .send(Command::draw(NewTypstDrawing::new(typst)))
        .await;

    Ok(())
}
