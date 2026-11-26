use iced::{
    Alignment, Element, Font, font,
    widget::{center, column, text},
};
use jiff::ZonedDateTime;

pub struct Screensaver {
    local_date_time: ZonedDateTime,
}

impl Default for Screensaver {
    fn default() -> Self {
        Self {
            local_date_time: ZonedDateTime::now(),
        }
    }
}

impl Screensaver {
    pub fn view<M: 'static>(&self) -> Element<'_, M> {
        let date = self.local_date_time.strftime("%a, %B %d").to_string();
        let time = self.local_date_time.strftime("%I:%M").to_string();

        center(
            column![
                text(date).size(40).font(Font {
                    weight: font::Weight::Semibold,
                    ..crate::INTER
                }),
                text(time).size(220).line_height(0.85).font(Font {
                    weight: font::Weight::Semibold,
                    ..crate::INTER
                }),
            ]
            .spacing(20)
            .align_x(Alignment::Center),
        )
        .into()
    }

    pub fn update(&mut self, _message: ()) {
        self.local_date_time = ZonedDateTime::now();
    }
}
