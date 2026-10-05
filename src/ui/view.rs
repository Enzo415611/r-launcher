use iced::Element;

use crate::{events::events::Events, state::state::AppState};

#[derive(Clone)]
pub enum Page {
    Login(LoginPages),
    Launcher,
}

#[derive(Clone)]
pub enum LoginPages {
    OfflineLogin,
    Home,
}

pub struct UiState {
    pub current_page: Page,
    pub name: String,
    pub login_mode: bool,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            current_page: Page::Login(LoginPages::Home),
            name: String::new(),
            login_mode: false,
        }
    }
}

impl AppState {
    pub fn view(&self) -> Element<'_, Events> {
        match &self.ui_state.current_page {
            Page::Login(p) => match p {
                LoginPages::Home => self.home_login_page(),
                LoginPages::OfflineLogin => self.offline_login_page(),
            },
            Page::Launcher => self.launcher_page(),
        }
    }
}
