use wavelet_core::app::{App, CardStyle};
use iced::{Color, color, gradient::Linear, widget::svg};

#[derive(Default)]
pub struct RadioBrowserApp;

impl App for RadioBrowserApp {
    fn icon(&self) -> svg::Handle {
        svg::Handle::from_memory(
            include_bytes!("../assets/icon.svg").as_slice(),
        )
    }

    fn title(&self) -> String {
        "RadioBrowser".to_owned()
    }

    fn card_style(&self) -> CardStyle {
        let linear = Linear::new(145.0_f32.to_radians())
            .add_stop(0.0, color!(0x44a2a1))
            .add_stop(0.45, color!(0x23263e));

        CardStyle {
            background: linear.into(),
            foreground: Color::WHITE,
        }
    }
}
