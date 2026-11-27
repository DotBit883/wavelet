use slint::Image;
use wavelet_core::app::{App, CardStyle, rgb, svg_icon};

#[derive(Default)]
pub struct JellyfinApp;

impl App for JellyfinApp {
    fn icon(&self) -> Image {
        svg_icon(include_bytes!("../assets/icon.svg"))
    }

    fn title(&self) -> String {
        "Jellyfin".to_owned()
    }

    fn card_style(&self) -> CardStyle {
        CardStyle::gradient(120.0, rgb(0xAA5CC3), rgb(0x00A4DC), rgb(0xFFFFFF))
    }
}
