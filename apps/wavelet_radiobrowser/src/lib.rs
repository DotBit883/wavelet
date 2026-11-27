use slint::Image;
use wavelet_core::app::{App, CardStyle, rgb, svg_icon};

#[derive(Default)]
pub struct RadioBrowserApp;

impl App for RadioBrowserApp {
    fn icon(&self) -> Image {
        svg_icon(include_bytes!("../assets/icon.svg"))
    }

    fn title(&self) -> String {
        "RadioBrowser".to_owned()
    }

    fn card_style(&self) -> CardStyle {
        CardStyle::gradient(150.0, rgb(0x44A2A1), rgb(0x23263E), rgb(0xFFFFFF)).ending_at(0.45)
    }
}
