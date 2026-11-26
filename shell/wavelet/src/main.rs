mod home;
mod screensaver;

use wavelet_ui::fonts::*;
use iced::{Element, Event, Size, Subscription, Theme, color, keyboard, mouse, theme::Palette, window};
use lucide_icons::LUCIDE_FONT_BYTES;

struct AppState {
    screensaver: screensaver::Screensaver,
    home: home::Home,
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
    Home(home::HomeMessage),
}

impl AppState {
    fn view(&self) -> Element<'_, AppMessage> {
        if self.is_awake {
            self.home.view().map(AppMessage::Home)
        } else {
            self.screensaver.view()
        }
    }

    fn update(&mut self, message: AppMessage) {
        // any input (including interacting with the home screen) wakes the app
        self.is_awake = true;

        if let AppMessage::Home(message) = message {
            self.home.update(message);
        } else if self.is_awake {
            self.home.tick();
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

    fn theme(&self) -> Theme {
        let palette = Palette {
            background: color!(0xF5F7FB),
            text: color!(0x232A48),
            primary: color!(0x0680F7),
            success: color!(0x12664f),
            warning: color!(0xb77e33),
            danger: color!(0xc3423f),
        };

        Theme::custom("Wavelet".to_string(), palette)
    }
}

fn main() -> iced::Result {
    iced::application(AppState::default, AppState::update, AppState::view)
        .subscription(AppState::subscription)
        .theme(AppState::theme)
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
