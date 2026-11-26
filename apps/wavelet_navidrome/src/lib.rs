use wavelet_core::app::{App, CardStyle};
use iced::{Color, color, gradient::Linear, widget::svg};

#[derive(Default)]
pub struct NavidromeApp;

impl App for NavidromeApp {
    fn icon(&self) -> svg::Handle {
        svg::Handle::from_memory(
            include_bytes!("../assets/icon.svg").as_slice(),
        )
    }

    fn title(&self) -> String {
        "Navidrome".to_owned()
    }

    fn card_style(&self) -> CardStyle {
        let linear = Linear::new(150.0_f32.to_radians())
            .add_stop(0.0, color!(0x3DA9FC))
            .add_stop(1.0, color!(0x0B5FD1));

        CardStyle {
            background: linear.into(),
            foreground: Color::WHITE,
        }
    }
}
