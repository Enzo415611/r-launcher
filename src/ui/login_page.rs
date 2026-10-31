use iced::{
    Alignment, Element, Length,
    widget::{button, column, container, pick_list, row, text, text_input},
};

use crate::{
    events::{events::Events, login_events::LoginEvents, ui_events::UiEvents},
    state::state::AppState,
    ui::view::{LoginPages, Page},
};

impl AppState {
    pub fn home_login_page(&self) -> Element<'_, Events> {
        // let online_login = button(text("Online").center())
        //     .on_press(Events::Ui(UiEvents::ChangePage(Page::Login(
        //         LoginPages::OnlineLogin,
        //     ))))
        //     .width(Length::Fill);

        let offline_login = button(text("Offline").center())
            .on_press(Events::Ui(UiEvents::ChangePage(Page::Login(
                LoginPages::OfflineLogin,
            ))))
            .width(Length::Fill);

        let accounts_list = if self.launcher_state.offline_accounts.is_empty() {
            None
        } else {
            Some(
                row![
                    pick_list(
                        self.launcher_state.offline_accounts.as_ref(),
                        self.launcher_state.current_account.as_ref(),
                        |a| Events::Ui(UiEvents::AccountSelected(a)),
                    )
                    .width(Length::Fill),
                    button(text("Remove").center())
                        .width(Length::Fill)
                        .on_press(Events::Login(LoginEvents::RemoveAccount))
                ]
                .spacing(3)
                .align_y(Alignment::Center),
            )
        };

        let launcher_button = button(text("Launcher").center())
            .on_press(Events::Ui(UiEvents::ChangePage(Page::Launcher)))
            .width(Length::Fill);

        let content = column![offline_login, accounts_list, launcher_button]
            .align_x(Alignment::Center)
            .width(200)
            .spacing(10);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center(Length::Fill)
            .into()
    }

    pub fn offline_login_page(&self) -> Element<'_, Events> {
        let user_name_input = text_input("Username", &self.ui_state.username)
            .width(300)
            .on_input(|e| Events::Ui(UiEvents::InputName(e)));

        let create_button = button("Create").on_press(Events::Login(LoginEvents::CreateAccount));
        let content = row![
            button("X").on_press(Events::Ui(UiEvents::ChangePage(Page::Login(
                LoginPages::Home
            )))),
            user_name_input,
            create_button
        ]
        .spacing(10);

        container(content).center(Length::Fill).into()
    }

    pub fn online_login_page(&self) -> Element<'_, Events> {
        let content = column![button("Open Browser")];
        container(content).into()
    }
}
