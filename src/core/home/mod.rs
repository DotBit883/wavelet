mod status_bar;

use crate::apps::{App, card, jellyfin::JellyfinApp, navidrome::NavidromeApp};
use iced::{
    Alignment, Element, Length, Padding, Theme,
    alignment::Horizontal,
    widget::{center, column, container, row, svg},
};

pub struct Home {
    status_bar: status_bar::StatusBar,
    apps: Vec<Box<dyn App>>,
}

impl Default for Home {
    fn default() -> Self {
        Self {
            status_bar: Default::default(),
            apps: vec![
                Box::new(JellyfinApp::default()),
                Box::new(NavidromeApp::default()),
            ],
        }
    }
}

impl Home {
    pub fn view<M: 'static>(&self) -> Element<'_, M> {
        let app_prev_page = container(prev_page())
            .width(Length::FillPortion(1))
            .padding(Padding {
                left: 13.0,
                ..Padding::ZERO
            })
            .align_x(Horizontal::Left);

        let apps = row(self.apps.iter().map(|app| card(app.as_ref()))).spacing(9.0);

        let app_next_page = container(next_page())
            .width(Length::FillPortion(1))
            .padding(Padding {
                right: 13.0,
                ..Padding::ZERO
            })
            .align_x(Horizontal::Right);

        column![
            self.status_bar.view(),
            center(
                row![app_prev_page, apps, app_next_page]
                    .width(Length::Fill)
                    .spacing(16.0)
                    .align_y(Alignment::Center)
            )
        ]
        .align_x(Alignment::Start)
        .spacing(53.0)
        .padding(Padding {
            top: 40.0,
            right: 0.0,
            bottom: 142.0,
            left: 0.0,
        })
        .into()
    }

    pub fn update(&mut self, _message: ()) {
        self.status_bar.update(());
    }
}

fn prev_page<'a, M: 'a>() -> Element<'a, M> {
    let handle = svg::Handle::from_memory(include_bytes!("../../../assets/images/prev_page.svg"));

    svg(handle)
        .width(Length::Shrink)
        .height(Length::Shrink)
        .style(|theme: &Theme, _status| svg::Style {
            color: Some(theme.palette().primary),
        })
        .into()
}

fn next_page<'a, M: 'a>() -> Element<'a, M> {
    let handle = svg::Handle::from_memory(include_bytes!("../../../assets/images/next_page.svg"));

    svg(handle)
        .width(Length::Shrink)
        .height(Length::Shrink)
        .style(|theme: &Theme, _status| svg::Style {
            color: Some(theme.palette().primary),
        })
        .into()
}
