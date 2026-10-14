use iced::Task;

use crate::{
    events::events::Events,
    launcher::versions_api::{Versions, get_versions},
    state::state::AppState,
};

#[derive(Clone)]
pub enum LauncherEvents {
    GetVersions,
    GetVersionsFuture(Option<Versions>),
    VersionSelected(String),
}

impl AppState {
    pub fn launcher_events(&mut self, e: LauncherEvents) -> Task<Events> {
        match e {
            LauncherEvents::GetVersions => Task::perform(get_versions(), |e| {
                Events::Launcher(LauncherEvents::GetVersionsFuture(e))
            }),
            LauncherEvents::GetVersionsFuture(r) => {
                if let Some(v) = r {
                    self.launcher_state.version_state.version_selected =
                        Some(v.latest.release.to_string());

                    self.launcher_state.version_state.versions_vec =
                        v.versions.iter().map(|v| v.id.to_string()).collect();

                    self.launcher_state.version_state.versions = Some(v);
                }
                Task::none()
            }
            LauncherEvents::VersionSelected(v) => {
                self.launcher_state.version_state.version_selected = Some(v);
                Task::none()
            }
        }
    }
}
