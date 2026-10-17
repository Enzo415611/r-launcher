use iced::{
    Element,
    Length::{self, Fill},
    widget::{button, checkbox, column, container, row, space::horizontal, text},
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
            Some(
                button(text("Login")).on_press(Events::Ui(UiEvents::ChangePage(Page::Login(
                    LoginPages::Home,
                )))),
            )
        };

        let show_snapshot_versions =
            checkbox(self.launcher_state.version_state.show_snapshots_versions)
                .label("Show Snapshot Versions")
                .on_toggle(|b| Events::Ui(UiEvents::ShowSnapshotVersions(b)));

        let content = column![
            row![horizontal().width(Fill), account_button,],
            row![].height(Length::Fill),
            row![
                horizontal().width(Fill),
                column![show_snapshot_versions, self.versions_button()],
            ]
        ];

        container(content).padding(5).into()
    }
}
