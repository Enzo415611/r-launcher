use crate::ui::view::{LoginPages, Page};

pub struct UiState {
    pub current_theme: iced::Theme,
    pub current_page: Page,
    pub username: String,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            current_theme: iced::Theme::CatppuccinMocha,
            current_page: Page::Login(LoginPages::Home),
            username: String::new(),
        }
    }
}
