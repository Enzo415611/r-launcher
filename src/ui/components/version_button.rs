use iced::{Element, widget::pick_list};

use crate::{
    events::{events::Events, launcher_events::LauncherEvents},
    state::state::AppState,
};

impl AppState {
    pub fn versions_button(&self) -> Element<'_, Events> {
        pick_list(
            self.launcher_state.version_state.versions_vec.as_ref(),
            self.launcher_state.version_state.version_selected.as_ref(),
            |s| Events::Launcher(LauncherEvents::VersionSelected(s)),
        )
        .into()
    }
}
