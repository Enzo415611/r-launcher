use iced::Element;

use crate::{AppState, events::events::Events};

enum Page {
    Login,
    Launcher,
}

pub struct UiState {
    pub current_page: Page,
    pub name: String,
    pub login_mode: bool,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            current_page: Page::Login,
            name: String::new(),
            login_mode: false,
        }
    }
}

impl AppState {
    pub fn view(&self) -> Element<'_, Events> {
        match self.ui_state.current_page {
            Page::Login => self.login_page(),
            Page::Launcher => self.launcher_page(),
        }
    }
}
