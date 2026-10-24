use iced::widget::pane_grid;

use crate::ui::view::{LoginPages, Page};

pub struct UiState {
    pub current_theme: iced::Theme,
    pub current_page: Page,
    pub username: String,
    pub instances_pane: pane_grid::State<InstancePane>,
}

impl Default for UiState {
    fn default() -> Self {
        let (mut state, list_pane) = pane_grid::State::new(InstancePane::List);
        state.split(pane_grid::Axis::Vertical, list_pane, InstancePane::Create);
        Self {
            current_theme: iced::Theme::CatppuccinMocha,
            current_page: Page::Login(LoginPages::Home),
            username: String::new(),
            instances_pane: state,
        }
    }
}

pub enum InstancePane {
    List,
    Create,
}
