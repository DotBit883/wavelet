use crate::apps::{App, CardStyle};
use iced::{Color, gradient::Linear, widget::svg};

#[derive(Default)]
pub struct JellyfinApp;

impl App for JellyfinApp {
    fn icon(&self) -> svg::Handle {
        svg::Handle::from_memory(
            include_bytes!("../../../assets/images/icons/apps/jellyfin.svg").as_slice(),
        )
    }

    fn title(&self) -> String {
        "Jellyfin".to_owned()
    }

    fn card_style(&self) -> CardStyle {
        let linear = Linear::new(120.0_f32.to_radians())
            .add_stop(0.0, Color::from_rgb8(170, 92, 195))
            .add_stop(1.0, Color::from_rgb8(0, 164, 220));

        CardStyle {
            background: linear.into(),
            foreground: Color::WHITE,
        }
    }
}
