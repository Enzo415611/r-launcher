mod ui;

use iced::Task;

use crate::ui::view::UiState;

fn main() -> iced::Result {
    iced::application(AppState::default, AppState::update, AppState::view)
        .theme(|a: &AppState| a.current_theme.to_owned())
        .run()
}

pub struct AppState {
    current_theme: iced::Theme,
    ui_state: UiState,
}

pub enum Events {}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_theme: iced::Theme::Dark,
            ui_state: UiState::default(),
        }
    }
}

impl AppState {
    fn update(&mut self, event: Events) -> Task<Events> {
        match event {}
    }
}
