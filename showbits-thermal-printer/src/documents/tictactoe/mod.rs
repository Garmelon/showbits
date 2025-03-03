use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};

use crate::server::{Server, somehow};

#[derive(Serialize)]
struct Data {
    feed: bool,
}

#[derive(Deserialize)]
pub struct FormData {
    pub feed: Option<bool>,
}

pub async fn post(server: State<Server>, Form(form): Form<FormData>) -> somehow::Result<()> {
    let data = Data {
        feed: form.feed.unwrap_or(true),
    };

    let typst = super::typst_with_lib()
        .with_json("/data.json", &data)
        .with_main_file(include_str!("main.typ"));

    server.print_typst(typst).await
}
