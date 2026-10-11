use crate::state::state::AppState;

mod config;
mod db;
mod events;
mod launcher;
mod state;
mod ui;

fn main() -> iced::Result {
    iced::application(AppState::new, AppState::update, AppState::view)
        .theme(|a: &AppState| a.current_theme.to_owned())
        .run()
}
