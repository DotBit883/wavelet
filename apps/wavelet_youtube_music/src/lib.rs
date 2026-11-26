use wavelet_core::app::{App, CardStyle};
use iced::{Color, color, widget::svg};

#[derive(Default)]
pub struct YoutubeMusicApp;

impl App for YoutubeMusicApp {
    fn icon(&self) -> svg::Handle {
        svg::Handle::from_memory(
            include_bytes!("../assets/icon.svg").as_slice(),
        )
    }

    fn title(&self) -> String {
        "Youtube Music".to_owned()
    }

    fn card_style(&self) -> CardStyle {
        CardStyle {
            background: color!(0xFF0033).into(),
            foreground: Color::WHITE,
        }
    }
}
