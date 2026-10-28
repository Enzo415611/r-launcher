use iced::{
    Alignment, Element, Length,
    widget::{checkbox, pick_list, row},
};

use crate::{
    events::{
        events::Events,
        instance_events::{
            create::CreateInstanceEvent, edit::EditInstanceEvent, instance_events::InstanceEvents,
        },
        ui_events::UiEvents,
    },
    state::state::AppState,
};

impl AppState {
    pub fn versions_button(&self, is_edit_mode: bool) -> Element<'_, Events> {
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
                        if is_edit_mode {
                            self.instances_state.instance_edit.mc_version.as_ref()
                        } else {
                            self.instances_state.create_instance.mc_version.as_ref()
                        },
                        move |v| if is_edit_mode {
                            Events::Instance(InstanceEvents::EditInstanceEvents(
                                EditInstanceEvent::Version(v),
                            ))
                        } else {
                            Events::Instance(InstanceEvents::CreateInstanceEvents(
                                CreateInstanceEvent::Version(v),
                            ))
                        },
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
                        if is_edit_mode {
                            self.instances_state.instance_edit.mc_version.as_ref()
                        } else {
                            self.instances_state.create_instance.mc_version.as_ref()
                        },
                        move |v| {
                            if is_edit_mode {
                                Events::Instance(InstanceEvents::EditInstanceEvents(
                                    EditInstanceEvent::Version(v),
                                ))
                            } else {
                                Events::Instance(InstanceEvents::CreateInstanceEvents(
                                    CreateInstanceEvent::Version(v),
                                ))
                            }
                        }
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
