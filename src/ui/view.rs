use iced::Element;

use crate::{AppState, Events};

enum Page {
    Login,
    Launcher,
}

pub struct UiState {
    current_page: Page,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            current_page: Page::Login,
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
