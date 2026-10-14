use iced::Task;

use crate::{
    db::{
        dto::versions_dto::{LatestDto, VersionDto, VersionsDto},
        service::versions_service::{get_versions_by_id, save_versions},
    },
    events::events::Events,
    launcher::versions_api::{Latest, Version, Versions, get_versions},
    state::state::AppState,
};

#[derive(Clone)]
pub enum LauncherEvents {
    GetVersions,
    GetVersionsFuture(Option<Versions>),
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
                    self.launcher_state.version_state.version_selected =
                        Some(v.latest.release.to_string());

                    self.launcher_state.version_state.versions_vec =
                        v.versions.iter().map(|v| v.id.to_string()).collect();

                    self.launcher_state.version_state.versions = Some(v.clone());

                    if let Some(db) = &self.db {
                        let versions_dto = VersionsDto {
                            latest: Some(LatestDto {
                                release: v.latest.release,
                                snapshot: v.latest.snapshot,
                            }),
                            versions: v
                                .versions
                                .into_iter()
                                .map(|v| VersionDto {
                                    id: v.id,
                                    version_type: v.version_type,
                                    url: Some(v.url),
                                    time: Some(v.time),
                                    release_time: Some(v.release_time),
                                    sha1: Some(v.sha1),
                                    compliance_level: Some(v.compliance_level),
                                })
                                .collect(),
                        };
                        println!("com net");
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
                self.launcher_state.version_state.version_selected = Some(v);
                Task::none()
            }
            LauncherEvents::SaveVersionsDb(_) => Task::none(),
            LauncherEvents::GetVersionsById(r) => {
                if let Some(v) = r {
                    self.launcher_state.version_state.versions_vec =
                        v.versions.iter().map(|v| v.id.clone()).collect();

                    let latest = v.latest.unwrap_or_default();

                    self.launcher_state.version_state.versions = Some(Versions {
                        latest: Latest {
                            release: latest.release,
                            snapshot: latest.snapshot,
                        },
                        versions: v
                            .versions
                            .into_iter()
                            .map(|v| Version {
                                id: v.id,
                                version_type: v.version_type,
                                url: v.url.unwrap_or_default(),
                                time: v.time.unwrap_or_default(),
                                release_time: v.release_time.unwrap_or_default(),
                                compliance_level: v.compliance_level.unwrap_or_default(),
                                sha1: v.sha1.unwrap_or_default(),
                            })
                            .collect(),
                    });
                }
                Task::none()
            }
        }
    }
}
