mod apps;
mod core;

use iced::{Element, Event, Font, Size, Subscription, keyboard, mouse, window};
use lucide_icons::LUCIDE_FONT_BYTES;

// Fonts
const INTER_LIGHT_BYTES: &[u8] = include_bytes!("../assets/fonts/inter_light.ttf");
const INTER_NORMAL_BYTES: &[u8] = include_bytes!("../assets/fonts/inter_normal.ttf");
const INTER_SEMIBOLD_BYTES: &[u8] = include_bytes!("../assets/fonts/inter_semibold.ttf");
const INTER_SEMIBOLD_ITALIC_BYTES: &[u8] =
    include_bytes!("../assets/fonts/inter_semibold_italic.ttf");
const INTER: Font = Font::with_name("Inter 18pt");

struct AppState {
    screensaver: core::screensaver::Screensaver,
    home: core::home::Home,
    is_awake: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            screensaver: Default::default(),
            home: Default::default(),
            is_awake: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum AppMessage {
    GlobalMouseMoved,
    GlobalMousePressed,
    GlobalKeyPressed,
}

impl AppState {
    fn view(&self) -> Element<'_, AppMessage> {
        if self.is_awake {
            self.home.view()
        } else {
            self.screensaver.view()
        }
    }

    fn update(&mut self, message: AppMessage) {
        // wake-up stuff
        self.is_awake = message == AppMessage::GlobalMouseMoved
            || message == AppMessage::GlobalMousePressed
            || message == AppMessage::GlobalKeyPressed;

        if self.is_awake {
            self.home.update(());
        } else {
            self.screensaver.update(());
        }
    }

    fn subscription(&self) -> Subscription<AppMessage> {
        iced::event::listen_with(|event, _status, _window| match event {
            Event::Mouse(mouse::Event::CursorMoved { .. }) => Some(AppMessage::GlobalMouseMoved),
            Event::Mouse(mouse::Event::ButtonPressed(_)) => Some(AppMessage::GlobalMousePressed),
            Event::Keyboard(keyboard::Event::KeyPressed { .. }) => {
                Some(AppMessage::GlobalKeyPressed)
            }
            _ => None,
        })
    }
}

fn main() -> iced::Result {
    iced::application(AppState::default, AppState::update, AppState::view)
        .subscription(AppState::subscription)
        .window(window::Settings {
            size: Size::new(800.0, 480.0),
            resizable: false,
            ..Default::default()
        })
        .title("wavelet")
        .font(INTER_LIGHT_BYTES)
        .font(INTER_NORMAL_BYTES)
        .font(INTER_SEMIBOLD_BYTES)
        .font(INTER_SEMIBOLD_ITALIC_BYTES)
        .font(LUCIDE_FONT_BYTES)
        .default_font(INTER)
        .run()
}
