use iced::Task;

use crate::{
    db::dto::user_dto::UserDto, events::events::Events, state::state::AppState, ui::view::Page,
};

#[derive(Clone)]
pub enum UiEvents {
    InputName(String),
    ChangePage(Page),
    AccountSelected(UserDto),
    ShowSnapshotVersions(bool),
}

impl AppState {
    pub fn ui_events(&mut self, e: UiEvents) -> Task<Events> {
        match e {
            UiEvents::InputName(s) => {
                self.ui_state.username = s;
                Task::none()
            }
            UiEvents::ChangePage(p) => {
                self.ui_state.current_page = p;
                Task::none()
            }
            UiEvents::AccountSelected(a) => {
                self.launcher_state.current_account = Some(a);
                self.save_config();
                Task::none()
            }
            UiEvents::ShowSnapshotVersions(b) => {
                self.launcher_state.version_state.show_snapshots_versions = b;
                Task::none()
            }
        }
    }
}
