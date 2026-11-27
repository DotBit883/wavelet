use std::time::{Duration, Instant};

use jiff::ZonedDateTime;
use slint::{ComponentHandle, ModelRc, VecModel};
use wavelet_core::app::App;

use crate::{AppEntry, Clock, MainWindow, Screen};

const APPS_PER_PAGE: usize = 4;

/// How long without input before the home screen gives way to the screensaver.
const IDLE_TIMEOUT: Duration = Duration::from_secs(60);

/// Application state and navigation logic. The UI itself lives in `ui/*.slint`;
/// after every change, `sync` pushes the relevant state into it.
pub struct Shell {
    apps: Vec<Box<dyn App>>,
    screen: Screen,
    page: usize,
    open_app: Option<usize>,
    last_activity: Instant,
}

impl Shell {
    pub fn new(apps: Vec<Box<dyn App>>) -> Self {
        Self {
            apps,
            screen: Screen::Home,
            page: 0,
            open_app: None,
            last_activity: Instant::now(),
        }
    }

    fn page_count(&self) -> usize {
        self.apps.len().div_ceil(APPS_PER_PAGE).max(1)
    }

    // --- events -----------------------------------------------------------

    /// Any touch, pointer or key input.
    pub fn activity(&mut self) -> bool {
        self.last_activity = Instant::now();

        if self.screen == Screen::Screensaver {
            self.screen = Screen::Home;
            return true
        }

        false
    }

    /// Called once per second.
    pub fn tick(&mut self) -> bool {
        // Only the home screen times out; an open app stays open.
        if self.screen == Screen::Home && self.last_activity.elapsed() >= IDLE_TIMEOUT {
            self.screen = Screen::Screensaver;
            return true
        }

        false
    }

    pub fn next_page(&mut self) -> bool {
        let new = (self.page + 1).min(self.page_count() - 1);
        std::mem::replace(&mut self.page, new) != new
    }

    pub fn previous_page(&mut self) -> bool {
        let new = self.page.saturating_sub(1);
        std::mem::replace(&mut self.page, new) != new
    }

    pub fn go_to_page(&mut self, page: usize) -> bool {
        let new = page.min(self.page_count() - 1);
        std::mem::replace(&mut self.page, new) != new
    }

    pub fn open_app(&mut self, index: usize) -> bool {
        // Ignore stale indices rather than panicking later.
        if index < self.apps.len() {
            self.open_app = Some(index);
            self.screen = Screen::App;
            return true
        }

        false
    }

    pub fn back(&mut self) -> bool {
        self.open_app = None;
        self.screen = Screen::Home;
        true
    }

    // --- pushing state into the UI ----------------------------------------

    pub fn sync(&self, ui: &MainWindow) {
        let first = self.page * APPS_PER_PAGE;
        let page_apps: Vec<AppEntry> = self
            .apps
            .iter()
            .enumerate()
            .skip(first)
            .take(APPS_PER_PAGE)
            .map(|(index, app)| {
                let style = app.card_style();

                AppEntry {
                    index: index as i32,
                    title: app.title().into(),
                    icon: app.icon(),
                    gradient_angle: style.angle,
                    start: style.start,
                    end: style.end,
                    end_position: style.end_position,
                    foreground: style.foreground,
                }
            })
            .collect();

        ui.set_page_apps(ModelRc::new(VecModel::from(page_apps)));
        ui.set_page(self.page as i32);
        ui.set_page_count(self.page_count() as i32);
        ui.set_screen(self.screen);

        let title = self
            .open_app
            .and_then(|index| self.apps.get(index))
            .map(|app| app.title())
            .unwrap_or_default();
        ui.set_open_app_title(title.into());
    }

    pub fn sync_clock(ui: &MainWindow) {
        let now = ZonedDateTime::now();
        let clock = ui.global::<Clock>();

        clock.set_short_date(now.strftime("%a, %b %d").to_string().into());
        clock.set_long_date(now.strftime("%a, %B %d").to_string().into());
        clock.set_time(now.strftime("%I:%M").to_string().into());
    }
}
