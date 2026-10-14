use toasty::Db;

use crate::db::{
    dto::versions_dto::{LatestDto, VersionDto, VersionsDto},
    model::versions_model::Versions as VersionsModel,
};

pub async fn save_versions(mut db: Db) -> Option<VersionsDto> {
    if let Err(err) = VersionsModel::delete_by_id(&mut db, "global").await {
        eprintln!("{}", err);
    }

    match VersionsModel::create().id("global").exec(&mut db).await {
        Ok(v) => Some(VersionsDto {
            latest: LatestDto {
                release: v.latest.release.clone(),
                snapshot: v.latest.snapshot.clone(),
            },
            versions: v
                .versions
                .into_iter()
                .map(|v| VersionDto {
                    id: v.id,
                    version_type: v.version_type,
                })
                .collect(),
        }),
        Err(err) => {
            eprintln!("{}", err);
            None
        }
    }
}

pub async fn get_versions(mut db: Db) -> Option<VersionsDto> {
    match VersionsModel::get_by_id(&mut db, "global").await {
        Ok(v) => Some(VersionsDto {
            latest: LatestDto {
                release: v.latest.release.clone(),
                snapshot: v.latest.snapshot.clone(),
            },
            versions: v
                .versions
                .into_iter()
                .map(|v| VersionDto {
                    id: v.id,
                    version_type: v.version_type,
                })
                .collect(),
        }),
        Err(err) => {
            eprintln!("{}", err);
            None
        }
    }
}
