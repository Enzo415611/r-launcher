use iced::{
    Element,
    widget::{column, container, row, text, text_input},
};

use crate::{
    AppState,
    events::{events::Events, ui_events::UiEvents},
};

impl AppState {
    pub fn login_page(&self) -> Element<'_, Events> {
        let view = if self.ui_state.login_mode {
            self.offline_login_page()
        } else {
            column![row![text("Login")]].into()
        };

        container(view).into()
    }

    fn offline_login_page(&self) -> Element<'_, Events> {
        let user_name_input = text_input("", self.ui_state.name.as_str())
            .on_input(|e| Events::UiEvents(UiEvents::InputName(e)));
        column![user_name_input].into()
    }
}
