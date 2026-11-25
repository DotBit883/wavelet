pub mod jellyfin;
pub mod navidrome;

use iced::{
    Alignment, Background, Border, Color, Element, Font,
    border::Radius,
    font,
    widget::{column, container, svg, text},
};

pub trait App {
    fn icon(&self) -> svg::Handle;

    fn title(&self) -> String;

    fn card_style(&self) -> CardStyle;
}

pub fn card<'a, M: 'a>(app: &'a dyn App) -> Element<'a, M> {
    let content = column![
        svg(app.icon())
            .width(64)
            .height(64)
            .style(move |_theme, _status| svg::Style {
                color: Some(app.card_style().foreground),
            }),
        text(app.title())
            .size(18)
            .style(|_theme| text::Style {
                color: Some(app.card_style().foreground),
            })
            .font(Font {
                weight: font::Weight::Medium,
                ..crate::INTER
            }),
    ]
    .spacing(37)
    .align_x(Alignment::Center);

    container(content)
        .width(158)
        .height(207)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .style(move |_theme| container::Style {
            background: Some(app.card_style().background),
            border: Border::default().rounded(Radius::new(10.0)),
            ..Default::default()
        })
        .into()
}

pub struct CardStyle {
    pub background: Background,
    pub foreground: Color,
}
