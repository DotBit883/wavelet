mod shell;

use std::{cell::RefCell, rc::Rc, time::Duration};

use slint::{ComponentHandle, Timer, TimerMode};
use wavelet_core::app::App;

use shell::Shell;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let apps: Vec<Box<dyn App>> = vec![
        // TODO (dot): Apple Music
        // TODO (dot): Deezer
        Box::new(wavelet_jellyfin::JellyfinApp),
        Box::new(wavelet_navidrome::NavidromeApp),
        Box::new(wavelet_radiobrowser::RadioBrowserApp),
        // TODO (dot): Spotify
        // TODO (dot): Tidal
        Box::new(wavelet_youtube_music::YoutubeMusicApp),
        Box::new(wavelet_settings::SettingsApp),
    ];

    let ui = MainWindow::new()?;
    let shell = Rc::new(RefCell::new(Shell::new(apps)));

    // Wraps a callback: run `f` on the shell, then push the new state to the UI.
    macro_rules! bind {
        ($register:ident, |$shell:ident $(, $arg:ident)*| $body:expr) => {{
            let ui_weak = ui.as_weak();
            let shell = shell.clone();
            ui.$register(move |$($arg),*| {
                let Some(ui) = ui_weak.upgrade() else { return };
                let mut $shell = shell.borrow_mut();
                if $body {
                    $shell.sync(&ui);
                }
            });
        }};
    }

    bind!(on_activity, |s| s.activity());
    bind!(on_previous, |s| s.previous_page());
    bind!(on_next, |s| s.next_page());
    bind!(on_go_to, |s, page| s.go_to_page(page as usize));
    bind!(on_open_app, |s, index| s.open_app(index as usize));
    bind!(on_back, |s| s.back());

    // Once per second: refresh the clock and check for idleness.
    let timer = Timer::default();
    {
        let ui_weak = ui.as_weak();
        let shell = shell.clone();
        timer.start(TimerMode::Repeated, Duration::from_secs(1), move || {
            let Some(ui) = ui_weak.upgrade() else { return };
            Shell::sync_clock(&ui);

            let mut shell = shell.borrow_mut();
            if shell.tick() {
                shell.sync(&ui);
            }
        });
    }

    Shell::sync_clock(&ui);
    shell.borrow().sync(&ui);

    ui.run()
}
