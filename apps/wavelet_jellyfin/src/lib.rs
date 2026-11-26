use wavelet_core::app::{App, CardStyle};
use iced::{Color, color, gradient::Linear, widget::svg};

#[derive(Default)]
pub struct JellyfinApp;

impl App for JellyfinApp {
    fn icon(&self) -> svg::Handle {
        svg::Handle::from_memory(
            include_bytes!("../assets/icon.svg").as_slice(),
        )
    }

    fn title(&self) -> String {
        "Jellyfin".to_owned()
    }

    fn card_style(&self) -> CardStyle {
        let linear = Linear::new(120.0_f32.to_radians())
            .add_stop(0.0, color!(0xAA5CC3))
            .add_stop(1.0, color!(0x00A4DC));

        CardStyle {
            background: linear.into(),
            foreground: Color::WHITE,
        }
    }
}
