use axum::{Form, extract::State};
use serde::{Deserialize, Serialize};

use crate::server::{Server, somehow};

#[derive(Serialize)]
struct Data {
    covers: usize,
    patterns: usize,
    bad_covers: usize,
    bad_patterns: usize,
    seed: i64,
    mode: Option<String>,
    feed: bool,
}

#[derive(Deserialize)]
pub struct FormData {
    pub seed: Option<i64>,
    pub mode: Option<String>,
    pub feed: Option<bool>,
}

pub async fn post(server: State<Server>, Form(form): Form<FormData>) -> somehow::Result<()> {
    let seed = form.seed.unwrap_or_else(rand::random);

    let data = Data {
        covers: showbits_assets::EGG_COVERS.len(),
        patterns: showbits_assets::EGG_PATTERNS.len(),
        bad_covers: showbits_assets::EGG_BAD_COVERS.len(),
        bad_patterns: showbits_assets::EGG_BAD_PATTERNS.len(),
        seed,
        mode: form.mode,
        feed: form.feed.unwrap_or(true),
    };

    let mut typst = super::typst_with_lib()
        .with_json("/data.json", &data)
        .with_main_file(include_str!("main.typ"));

    for (i, cover) in showbits_assets::EGG_COVERS.iter().enumerate() {
        typst.add_file(format!("/eggs/good/cover_{i:02}.png"), *cover);
    }

    for (i, pattern) in showbits_assets::EGG_PATTERNS.iter().enumerate() {
        typst.add_file(format!("/eggs/good/pattern_{i:02}.png"), *pattern);
    }

    for (i, cover) in showbits_assets::EGG_BAD_COVERS.iter().enumerate() {
        typst.add_file(format!("/eggs/bad/cover_{i:02}.png"), *cover);
    }

    for (i, pattern) in showbits_assets::EGG_BAD_PATTERNS.iter().enumerate() {
        typst.add_file(format!("/eggs/bad/pattern_{i:02}.png"), *pattern);
    }

    server.print_typst(typst).await
}
