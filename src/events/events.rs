use iced::Task;

use crate::{events::ui_events::UiEvents, ui::view::UiState};

pub struct AppState {
    pub current_theme: iced::Theme,
    pub ui_state: UiState,
}

#[derive(Clone)]
pub enum Events {
    UiEvents(UiEvents),
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_theme: iced::Theme::Dark,
            ui_state: UiState::default(),
        }
    }
}

impl AppState {
    pub fn update(&mut self, event: Events) -> Task<Events> {
        match event {
            Events::UiEvents(e) => match e {
                UiEvents::InputName(s) => {
                    self.ui_state.name = s;
                    Task::none()
                }
            },
        }
    }
}
