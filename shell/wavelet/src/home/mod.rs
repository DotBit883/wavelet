mod status_bar;

use iced::{
    Alignment, Element, Length, Padding,
    alignment::Horizontal,
    widget::{center, column, container, row},
};
use wavelet_core::app::{App, card};
use wavelet_ui::widgets::{dot_button, next_page_button, prev_page_button};

const APPS_PER_PAGE: usize = 4;

pub struct Home {
    status_bar: status_bar::StatusBar,
    apps: Vec<Box<dyn App>>,
    current_page: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HomeMessage {
    Next,
    Previous,
    GoTo(usize),
}

impl Default for Home {
    fn default() -> Self {
        Self {
            status_bar: Default::default(),
            apps: vec![
                // - Apple Music
                // - Deezer
                Box::new(wavelet_jellyfin::JellyfinApp::default()),
                Box::new(wavelet_navidrome::NavidromeApp::default()),
                Box::new(wavelet_radiobrowser::RadioBrowserApp::default()),
                // - Spotify
                // - Tidal
                Box::new(wavelet_youtube_music::YoutubeMusicApp::default()),
                Box::new(wavelet_settings::SettingsApp::default()),
            ],
            current_page: 0,
        }
    }
}

impl Home {
    pub fn view(&self) -> Element<'_, HomeMessage> {
        let page_count = self.page_count();

        let page_apps = self
            .apps
            .chunks(APPS_PER_PAGE)
            .nth(self.current_page)
            .unwrap_or_default();

        let apps = container(row(page_apps.iter().map(|app| card(app.as_ref()))).spacing(9.0))
            .width(Length::Fill)
            .align_x(Horizontal::Left);

        let slide_controls = center(
            row((0..page_count).map(|page| {
                let active = page == self.current_page;
                dot_button(
                    if active { 16.0 } else { 10.0 },
                    active,
                    HomeMessage::GoTo(page),
                )
            }))
            .spacing(10.0)
            .align_y(Alignment::Center),
        )
        .padding(Padding {
            top: 23.0,
            ..Padding::ZERO
        });

        column![
            container(self.status_bar.view()).padding(Padding {
                bottom: 53.0,
                ..Padding::ZERO
            }),
            row![
                prev_page_button(self.current_page > 0, HomeMessage::Previous),
                apps,
                next_page_button(self.current_page + 1 < page_count, HomeMessage::Next),
            ]
            .width(Length::Fill)
            .spacing(16.0)
            .align_y(Alignment::Center),
            slide_controls,
        ]
        .padding(Padding {
            top: 40.0,
            right: 0.0,
            bottom: 103.0,
            left: 0.0,
        })
        .into()
    }

    fn page_count(&self) -> usize {
        self.apps.len().div_ceil(APPS_PER_PAGE).max(1)
    }

    pub fn update(&mut self, message: HomeMessage) {
        let last_page = self.page_count() - 1;

        match message {
            HomeMessage::Next => self.current_page = (self.current_page + 1).min(last_page),
            HomeMessage::Previous => self.current_page = self.current_page.saturating_sub(1),
            HomeMessage::GoTo(page) => self.current_page = page.min(last_page),
        }
    }

    pub fn tick(&mut self) {
        self.status_bar.update(());
    }
}
