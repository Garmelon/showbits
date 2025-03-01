use showbits_typst::Typst;

pub mod image;
pub mod text;

fn typst_with_lib() -> Typst {
    Typst::new()
        .with_file("/lib.typ", include_str!("documents/lib.typ"))
        .with_file("/plugin.wasm", include_bytes!("documents/plugin.wasm"))
}
