use slint::Image;
use wavelet_core::app::{App, CardStyle, rgb, svg_icon};

#[derive(Default)]
pub struct SettingsApp;

impl App for SettingsApp {
    fn icon(&self) -> Image {
        svg_icon(include_bytes!("../assets/icon.svg"))
    }

    fn title(&self) -> String {
        "Settings".to_owned()
    }

    fn card_style(&self) -> CardStyle {
        CardStyle::solid(rgb(0xFFFFFF), rgb(0x0680F7))
    }
}
