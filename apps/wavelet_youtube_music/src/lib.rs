use slint::Image;
use wavelet_core::app::{App, CardStyle, rgb, svg_icon};

#[derive(Default)]
pub struct YoutubeMusicApp;

impl App for YoutubeMusicApp {
    fn icon(&self) -> Image {
        svg_icon(include_bytes!("../assets/icon.svg"))
    }

    fn title(&self) -> String {
        "Youtube Music".to_owned()
    }

    fn card_style(&self) -> CardStyle {
        CardStyle::solid(rgb(0xFF0033), rgb(0xFFFFFF))
    }
}
