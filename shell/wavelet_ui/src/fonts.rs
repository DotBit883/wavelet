use iced::Font;

pub const INTER_LIGHT_BYTES: &[u8] = include_bytes!("../assets/fonts/inter_light.ttf");
pub const INTER_NORMAL_BYTES: &[u8] = include_bytes!("../assets/fonts/inter_normal.ttf");
pub const INTER_SEMIBOLD_BYTES: &[u8] = include_bytes!("../assets/fonts/inter_semibold.ttf");
pub const INTER_SEMIBOLD_ITALIC_BYTES: &[u8] =
    include_bytes!("../assets/fonts/inter_semibold_italic.ttf");
pub const INTER: Font = Font::with_name("Inter 18pt");
