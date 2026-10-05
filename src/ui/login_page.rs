use iced::{
    Alignment, Element,
    widget::{button, column, container, pick_list, row, text_input},
};

use crate::{
    db::user_dto::UserDto,
    events::{events::Events, ui_events::UiEvents},
    state::state::AppState,
    ui::view::{LoginPages, Page},
};

impl AppState {
    pub fn home_login_page(&self) -> Element<'_, Events> {
        let offline_login = button("Offline Account").on_press(Events::UiEvents(
            UiEvents::ChangePage(Page::Login(LoginPages::OfflineLogin)),
        ));
        let list = pick_list(
            self.launcher_state.offline_users.to_owned(),
            self.launcher_state.current_account.to_owned(),
            |a| Events::UiEvents(UiEvents::AccountSelected(a)),
        );

        container(column![row![offline_login, list]]).into()
    }

    pub fn offline_login_page(&self) -> Element<'_, Events> {
        let user_name_input = text_input("Username", self.ui_state.name.as_str())
            .on_input(|e| Events::UiEvents(UiEvents::InputName(e)));

        let create = button("Create");
        column![
            row![
                button("Login").on_press(Events::UiEvents(UiEvents::ChangePage(Page::Login(
                    LoginPages::Home
                ))))
            ]
            .align_y(Alignment::Start),
            user_name_input,
            create
        ]
        .into()
    }
}
