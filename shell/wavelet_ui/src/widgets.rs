use iced::{
    Border, Element, Length, Padding, Theme,
    widget::{Space, button, svg},
};

pub fn prev_page_button<'a, M: Clone + 'a>(enabled: bool, on_press: M) -> Element<'a, M> {
    page_button(
        include_bytes!("../assets/icons/prev_page.svg").as_slice(),
        on_press,
        enabled,
        Padding {
            left: 13.0,
            ..Padding::ZERO
        },
    )
}

pub fn next_page_button<'a, M: Clone + 'a>(enabled: bool, on_press: M) -> Element<'a, M> {
    page_button(
        include_bytes!("../assets/icons/next_page.svg").as_slice(),
        on_press,
        enabled,
        Padding {
            right: 13.0,
            ..Padding::ZERO
        },
    )
}

fn page_button<'a, M: Clone + 'a>(
    icon: &'static [u8],
    message: M,
    enabled: bool,
    padding: Padding,
) -> Element<'a, M> {
    const ARROW_WIDTH: f32 = 41.0;
    const ARROW_HEIGHT: f32 = 78.0;

    if !enabled {
        return Space::new()
            .width(ARROW_WIDTH + padding.x())
            .height(ARROW_HEIGHT)
            .into();
    }

    let arrow = svg(svg::Handle::from_memory(icon))
        .width(Length::Shrink)
        .height(Length::Shrink)
        .style(move |theme: &Theme, status| svg::Style {
            color: Some(match status {
                svg::Status::Idle => theme.palette().primary,
                svg::Status::Hovered => theme.extended_palette().primary.strong.color,
            }),
        });

    button(arrow)
        .padding(padding)
        .style(|_theme, _status| button::Style::default())
        .on_press(message)
        .into()
}

pub fn dot_button<'a, M: Clone + 'a>(
    diameter: f32,
    use_primary: bool,
    on_press: M,
) -> Element<'a, M> {
    button(Space::new())
        .width(diameter)
        .height(diameter)
        .padding(0)
        .style(move |theme: &Theme, status| {
            let palette = theme.extended_palette();
            let interacted = matches!(status, button::Status::Hovered | button::Status::Pressed);

            let color = match (use_primary, interacted) {
                (true, false) => palette.primary.base.color,
                (true, true) => palette.primary.strong.color,
                (false, false) => palette.background.strong.color,
                (false, true) => palette.background.stronger.color,
            };

            button::Style {
                background: Some(color.into()),
                border: Border::default().rounded(diameter / 2.0),
                ..button::Style::default()
            }
        })
        .on_press(on_press)
        .into()
}
