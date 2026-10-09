use iced::{
    Element,
    Length::Fill,
    widget::{button, column, container, row, space::horizontal, text},
};

use crate::{
    events::{events::Events, ui_events::UiEvents},
    state::state::AppState,
    ui::view::{LoginPages, Page},
};

impl AppState {
    pub fn launcher_page(&self) -> Element<'_, Events> {
        let account_button = if let Some(u) = &self.launcher_state.current_account {
            Some(
                button(text(format!("{}", u.username.to_string()))).on_press(Events::Ui(
                    UiEvents::ChangePage(Page::Login(LoginPages::Home)),
                )),
            )
        } else {
            None
        };
        container(column![row![horizontal().width(Fill), account_button]])
            .padding(5)
            .into()
    }
}
