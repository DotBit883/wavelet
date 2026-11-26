use iced::{
    Alignment, Element, Font, Length, Padding,
    alignment::Horizontal,
    font,
    widget::{container, row, text},
};
use jiff::ZonedDateTime;
use lucide_icons::iced::{icon_ethernet_port, icon_refresh_cw, icon_volume_2};

pub struct StatusBar {
    local_date_time: ZonedDateTime,
}

impl Default for StatusBar {
    fn default() -> Self {
        Self {
            local_date_time: ZonedDateTime::now(),
        }
    }
}

impl StatusBar {
    pub fn view<M: 'static>(&self) -> Element<'_, M> {
        let date = self.local_date_time.strftime("%a, %b %d").to_string();
        let time = self.local_date_time.strftime("%I:%M").to_string();

        let left = container(text(date).size(20).font(Font {
            weight: font::Weight::Light,
            ..crate::INTER
        }))
        .width(Length::FillPortion(1))
        .align_x(Horizontal::Left);

        let center = text(time).size(43).line_height(0.85).font(Font {
            weight: font::Weight::Normal,
            ..crate::INTER
        });

        let right = container(
            row![
                icon_refresh_cw().size(23),
                icon_volume_2().size(23),
                icon_ethernet_port().size(23),
            ]
            .spacing(6.0),
        )
        .width(Length::FillPortion(1))
        .align_x(Horizontal::Right);

        row![left, center, right]
            .align_y(Alignment::End)
            .padding(Padding {
                top: 0.0,
                right: 76.0,
                bottom: 0.0,
                left: 76.0,
            })
            .into()
    }

    pub fn update(&mut self, _message: ()) {
        self.local_date_time = ZonedDateTime::now();
    }
}
