use showbits_typst::Typst;

pub use self::{image::*, text::*};

mod image;
mod text;

fn typst_with_lib() -> Typst {
    Typst::new()
        .with_file("/lib.typ", include_str!("documents/lib.typ"))
        .with_file("/plugin.wasm", include_bytes!("documents/plugin.wasm"))
}
