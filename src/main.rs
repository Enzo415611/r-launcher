mod events;
mod ui;

use iced::Task;

use crate::{events::events::AppState, ui::view::UiState};

fn main() -> iced::Result {
    iced::application(AppState::default, AppState::update, AppState::view)
        .theme(|a: &AppState| a.current_theme.to_owned())
        .run()
}
