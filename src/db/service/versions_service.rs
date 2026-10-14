use toasty::Db;

use crate::db::{
    dto::versions_dto::{LatestDto, VersionDto, VersionsDto},
    model::versions_model::{Latest, Version, Versions as VersionsModel},
};

pub async fn save_versions(mut db: Db, v: VersionsDto) -> Option<VersionsDto> {
    if let Err(err) = VersionsModel::delete_by_id(&mut db, "global").await {
        eprintln!("{}", err);
    }
    let l = v.latest.unwrap_or_default();
    let v: Vec<Version> = v
        .versions
        .into_iter()
        .map(|v| Version {
            id: v.id,
            version_type: v.version_type,
            url: v.url,
            time: v.time,
            sha1: v.sha1,
            release_time: v.release_time,
            compliance_level: v.compliance_level,
        })
        .collect();

    match VersionsModel::create()
        .id("global")
        .latest(Latest {
            release: l.release,
            snapshot: l.snapshot,
        })
        .versions(v)
        .exec(&mut db)
        .await
    {
        Ok(v) => Some(VersionsDto {
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
                    url: v.url,
                    time: v.time,
                    sha1: v.sha1,
                    release_time: v.release_time,
                    compliance_level: v.compliance_level,
                })
                .collect(),
        }),
        Err(err) => {
            eprintln!("{}", err);
            None
        }
    }
}

pub async fn get_versions_by_id(mut db: Db) -> Option<VersionsDto> {
    match VersionsModel::get_by_id(&mut db, "global").await {
        Ok(v) => Some(VersionsDto {
            latest: Some(LatestDto {
                release: v.latest.release,
                snapshot: v.latest.snapshot.clone(),
            }),
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
        }),
        Err(err) => {
            eprintln!("{}", err);
            None
        }
    }
}
