use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};

use crate::{
    drawer::{Command, NewTypstDrawing},
    server::Server,
};

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub text: String,
    #[serde(default)]
    pub force_wrap: bool,
    #[serde(default)]
    pub feed: bool,
}

pub async fn post(server: State<Server>, request: Form<Data>) {
    let typst = super::typst_with_lib()
        .with_json("/data.json", &request.0)
        .with_main_file(include_str!("main.typ"));

    let _ = server
        .tx
        .send(Command::draw(NewTypstDrawing::new(typst)))
        .await;
}
