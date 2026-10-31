use crate::state::state::AppState;

mod auth;
mod config;
mod db;
mod events;
mod launcher;
mod state;
mod ui;

fn main() -> iced::Result {
    if let Err(err) = lighty_launcher::core::AppState::init("RLauncher") {
        eprintln!("{}", err);
    }
    iced::application(AppState::new, AppState::update, AppState::view)
        .theme(|a: &AppState| a.ui_state.current_theme.clone())
        .run()
}
