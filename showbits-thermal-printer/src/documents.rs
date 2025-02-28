use showbits_typst::Typst;

pub use self::text::*;

mod text;

fn typst_with_lib() -> Typst {
    Typst::new().with_file("/lib.typ", include_str!("documents/lib.typ"))
}
