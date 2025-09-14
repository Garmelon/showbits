use axum::{
    Form,
    extract::State,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

use crate::server::{Server, somehow};

#[derive(Serialize, Deserialize)]
struct ArticleInfo {
    title: String,
    categories: Vec<String>,
}

#[derive(Serialize)]
struct Data {
    day: u32,
    articles: Vec<ArticleInfo>,
    feed: bool,
}

#[derive(Deserialize)]
pub struct FormData {
    pub day: u32,
    pub feed: bool,
}

pub async fn post(server: State<Server>, Form(form): Form<FormData>) -> somehow::Result<Response> {
    let client = reqwest::Client::builder()
        .user_agent(crate::USER_AGENT)
        .build()?;

    let url = format!("https://static.catfishing.net/daily/{}.json", form.day);

    let articles = client
        .get(url)
        .send()
        .await?
        .json::<Vec<ArticleInfo>>()
        .await?;

    let data = Data {
        day: form.day,
        articles,
        feed: form.feed,
    };

    let typst = super::typst_with_lib()
        .with_json("/data.json", &data)
        .with_main_file(include_str!("main.typ"));

    server.print_typst(typst).await?;
    Ok(().into_response())
}
