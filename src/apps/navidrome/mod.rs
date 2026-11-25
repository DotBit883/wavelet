use crate::apps::{App, CardStyle};
use iced::{Color, gradient::Linear, widget::svg};

#[derive(Default)]
pub struct NavidromeApp;

impl App for NavidromeApp {
    fn icon(&self) -> svg::Handle {
        svg::Handle::from_memory(
            include_bytes!("../../../assets/images/icons/apps/navidrome.svg").as_slice(),
        )
    }

    fn title(&self) -> String {
        "Navidrome".to_owned()
    }

    fn card_style(&self) -> CardStyle {
        let linear = Linear::new(120.0_f32.to_radians())
            .add_stop(0.0, Color::from_rgb8(88, 175, 255))
            .add_stop(1.0, Color::from_rgb8(0, 132, 255));

        CardStyle {
            background: linear.into(),
            foreground: Color::WHITE,
        }
    }
}
