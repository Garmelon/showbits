use showbits_typst::Typst;

pub mod calendar;
pub mod cells;
pub mod chat;
pub mod egg;
pub mod image;
pub mod sunrise;
pub mod text;
pub mod tictactoe;
pub mod xkcd;

fn typst_with_lib() -> Typst {
    Typst::new().with_file("/lib/main.typ", include_str!("documents/lib/main.typ"))
}
