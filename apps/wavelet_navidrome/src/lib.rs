use slint::Image;
use wavelet_core::app::{App, CardStyle, rgb, svg_icon};

#[derive(Default)]
pub struct NavidromeApp;

impl App for NavidromeApp {
    fn icon(&self) -> Image {
        svg_icon(include_bytes!("../assets/icon.svg"))
    }

    fn title(&self) -> String {
        "Navidrome".to_owned()
    }

    fn card_style(&self) -> CardStyle {
        CardStyle::gradient(150.0, rgb(0x3DA9FC), rgb(0x0B5FD1), rgb(0xFFFFFF))
    }
}
