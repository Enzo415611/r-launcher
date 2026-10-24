use iced::{
    Alignment, Element,
    Length::{self, Fill},
    widget::{button, column, container, row, rule, space::horizontal, text},
};

use crate::{
    events::{events::Events, ui_events::UiEvents},
    state::state::AppState,
    ui::view::{LoginPages, Page},
};

impl AppState {
    pub fn launcher_page(&self) -> Element<'_, Events> {
        let content = column![
            row![horizontal().width(Fill), self.account_button(),],
            rule::horizontal(0.8),
            row![self.instances()]
                .align_y(Alignment::Start)
                .height(Length::Fill),
        ]
        .spacing(3);
        container(content).padding(5).into()
    }

    fn account_button(&self) -> Option<Element<'_, Events>> {
        if let Some(u) = &self.launcher_state.current_account {
            Some(
                button(text(format!("{}", u.username.to_string())))
                    .on_press(Events::Ui(UiEvents::ChangePage(Page::Login(
                        LoginPages::Home,
                    ))))
                    .into(),
            )
        } else {
            Some(
                button(text("Login"))
                    .on_press(Events::Ui(UiEvents::ChangePage(Page::Login(
                        LoginPages::Home,
                    ))))
                    .into(),
            )
        }
    }
}
