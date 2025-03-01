use showbits_typst::Typst;

pub mod cells;
pub mod egg;
pub mod image;
pub mod text;

fn typst_with_lib() -> Typst {
    Typst::new()
        .with_file("/lib/main.typ", include_str!("documents/lib/main.typ"))
        .with_file(
            "/lib/plugin.wasm",
            include_bytes!("documents/lib/plugin.wasm"),
        )
}
