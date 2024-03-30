pub const UNIFONT: &[u8] = include_bytes!("../data/unifont-15.1.05.otf");
pub const UNIFONT_JP: &[u8] = include_bytes!("../data/unifont_jp-15.1.05.otf");
pub const UNIFONT_UPPER: &[u8] = include_bytes!("../data/unifont_upper-15.1.05.otf");
pub const UNIFONT_NAME: &str = "Unifont";
pub const UNIFONT_SIZE: f32 = 16.0;

pub const EGG_COVER: &[u8] = include_bytes!("../data/egg_cover.png");
pub const EGG_PATTERNS: &[&[u8]] = &[
    include_bytes!("../data/egg_pattern_0.png"),
    include_bytes!("../data/egg_pattern_1.png"),
    include_bytes!("../data/egg_pattern_2.png"),
    include_bytes!("../data/egg_pattern_3.png"),
    include_bytes!("../data/egg_pattern_4.png"),
    include_bytes!("../data/egg_pattern_5.png"),
    include_bytes!("../data/egg_pattern_6.png"),
];
