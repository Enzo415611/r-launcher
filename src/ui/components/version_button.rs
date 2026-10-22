use iced::{
    Alignment, Element, Length,
    widget::{checkbox, pick_list, row},
};

use crate::{
    events::{events::Events, launcher_events::LauncherEvents, ui_events::UiEvents},
    state::state::AppState,
};

impl AppState {
    pub fn versions_button(&self) -> Element<'_, Events> {
        let show_snapshot_versions =
            checkbox(self.launcher_state.version_state.show_snapshots_versions)
                .label("Show Snapshot Versions")
                .width(Length::Fill)
                .on_toggle(|b| Events::Ui(UiEvents::ShowSnapshotVersions(b)));

        let content = if self.launcher_state.version_state.show_snapshots_versions {
            Some(
                row![
                    pick_list(
                        self.launcher_state.version_state.all_versions_vec.as_ref(),
                        self.instances_state.create_instance.mc_version.as_ref(),
                        |s| Events::Launcher(LauncherEvents::VersionSelected(s)),
                    )
                    .width(Length::Fill),
                    show_snapshot_versions
                ]
                .width(Length::Fill)
                .spacing(3)
                .align_y(Alignment::Center),
            )
        } else {
            Some(
                row![
                    pick_list(
                        self.launcher_state
                            .version_state
                            .release_versions_vec
                            .as_ref(),
                        self.instances_state.create_instance.mc_version.as_ref(),
                        |s| Events::Launcher(LauncherEvents::VersionSelected(s)),
                    )
                    .width(Length::Fill),
                    show_snapshot_versions
                ]
                .width(Length::Fill)
                .spacing(3)
                .align_y(Alignment::Center),
            )
        };
        content.into()
    }
}
