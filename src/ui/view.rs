use iced::Element;

use crate::{events::events::Events, state::state::AppState};

#[derive(Clone, Debug)]
pub enum Page {
    Login(LoginPages),
    Launcher,
}

#[derive(Clone, Debug)]
pub enum LoginPages {
    OfflineLogin,
    Home,
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
