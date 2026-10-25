use iced::Task;

use crate::{
    db::{
        dto::versions_dto::{LatestDto, VersionDto, VersionsDto},
        service::versions_service::{get_versions_by_id, save_versions},
    },
    events::events::Events,
    launcher::versions_api::get_versions,
    state::state::AppState,
};

#[derive(Clone, Debug)]
pub enum LauncherEvents {
    GetVersions,
    GetVersionsFuture(Option<VersionsDto>),
    VersionSelected(String),
    SaveVersionsDb(()),
    GetVersionsById(Option<VersionsDto>),
}

impl AppState {
    pub fn launcher_events(&mut self, e: LauncherEvents) -> Task<Events> {
        match e {
            LauncherEvents::GetVersions => Task::perform(get_versions(), |e| {
                Events::Launcher(LauncherEvents::GetVersionsFuture(e))
            }),
            LauncherEvents::GetVersionsFuture(r) => {
                if let Some(v) = r {
                    self.instances_state.create_instance.version = Some(v.latest.release.clone());
                    self.launcher_state.version_state.release_versions_vec = v
                        .versions
                        .iter()
                        .filter(|v| v.version_type == "release")
                        .map(|v| v.id.clone())
                        .collect::<Vec<String>>();

                    self.launcher_state.version_state.all_versions_vec =
                        v.versions.iter().map(|v| v.id.to_string()).collect();

                    self.launcher_state.version_state.versions = Some(v.clone());

                    if let Some(db) = &self.db {
                        let versions_dto = VersionsDto {
                            latest: LatestDto {
                                release: v.latest.release,
                                snapshot: v.latest.snapshot,
                            },
                            versions: v
                                .versions
                                .into_iter()
                                .map(|v| VersionDto {
                                    id: v.id,
                                    version_type: v.version_type,
                                    url: v.url,
                                    time: v.time,
                                    release_time: v.release_time,
                                    sha1: v.sha1,
                                    compliance_level: v.compliance_level,
                                })
                                .collect(),
                        };
                        return Task::perform(save_versions(db.clone(), versions_dto), |_| {
                            Events::Launcher(LauncherEvents::SaveVersionsDb(()))
                        });
                    }
                } else {
                    if let Some(db) = &self.db {
                        // if the API fails, I get the versions from the database
                        return Task::perform(get_versions_by_id(db.clone()), |v| {
                            Events::Launcher(LauncherEvents::GetVersionsById(v))
                        });
                    }
                }
                Task::none()
            }
            LauncherEvents::VersionSelected(v) => {
                self.instances_state.create_instance.version = Some(v);
                Task::none()
            }
            LauncherEvents::SaveVersionsDb(_) => Task::none(),
            LauncherEvents::GetVersionsById(r) => {
                if let Some(v) = r {
                    self.instances_state.create_instance.version = Some(v.latest.release.clone());
                    self.launcher_state.version_state.release_versions_vec = v
                        .versions
                        .iter()
                        .filter(|v| v.version_type == "release")
                        .map(|v| v.id.clone())
                        .collect::<Vec<String>>();

                    self.launcher_state.version_state.all_versions_vec =
                        v.versions.iter().map(|v| v.id.clone()).collect();

                    let latest = v.latest;

                    self.launcher_state.version_state.versions = Some(VersionsDto {
                        latest: LatestDto {
                            release: latest.release,
                            snapshot: latest.snapshot,
                        },
                        versions: v
                            .versions
                            .into_iter()
                            .map(|v| VersionDto {
                                id: v.id,
                                version_type: v.version_type,
                                url: v.url,
                                time: v.time,
                                release_time: v.release_time,
                                compliance_level: v.compliance_level,
                                sha1: v.sha1,
                            })
                            .collect(),
                    });
                }
                Task::none()
            }
        }
    }
}
