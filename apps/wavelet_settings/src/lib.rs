use iced::{Color, color, widget::svg};
use wavelet_core::app::{App, CardStyle};

#[derive(Default)]
pub struct SettingsApp;

impl App for SettingsApp {
    fn icon(&self) -> svg::Handle {
        svg::Handle::from_memory(include_bytes!("../assets/icon.svg").as_slice())
    }

    fn title(&self) -> String {
        "Settings".to_owned()
    }

    fn card_style(&self) -> CardStyle {
        CardStyle {
            background: Color::WHITE.into(),
            foreground: color!(0x0680f7),
        }
    }
}
