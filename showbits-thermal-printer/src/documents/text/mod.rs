use serde::{Deserialize, Serialize};
use showbits_typst::Typst;

#[derive(Serialize, Deserialize)]
pub struct Text {
    pub text: String,
    #[serde(default)]
    pub force_wrap: bool,
    #[serde(default)]
    pub feed: bool,
}

impl From<Text> for Typst {
    fn from(value: Text) -> Self {
        super::typst_with_lib()
            .with_json("/data.json", &value)
            .with_main_file(include_str!("main.typ"))
    }
}
