use iced::{
    Element,
    widget::{pick_list, row},
};

use crate::{
    events::{events::Events, launcher_events::LauncherEvents},
    state::state::AppState,
};

impl AppState {
    pub fn versions_button(&self) -> Element<'_, Events> {
        let list = if self.launcher_state.version_state.show_snapshots_versions {
            Some(pick_list(
                self.launcher_state.version_state.all_versions_vec.as_ref(),
                self.launcher_state.version_state.version_selected.as_ref(),
                |s| Events::Launcher(LauncherEvents::VersionSelected(s)),
            ))
        } else {
            Some(pick_list(
                self.launcher_state
                    .version_state
                    .release_versions_vec
                    .as_ref(),
                self.launcher_state.version_state.version_selected.as_ref(),
                |s| Events::Launcher(LauncherEvents::VersionSelected(s)),
            ))
        };
        list.into()
    }
}
