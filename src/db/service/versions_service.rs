use toasty::Db;

use crate::db::{
    dto::versions_dto::{LatestDto, VersionDto, VersionsDto},
    model::versions_model::Versions as VersionsModel,
};

pub async fn save_versions(mut db: Db) -> Option<VersionsDto> {
    match VersionsModel::create().exec(&mut db).await {
        Ok(v) => Some(VersionsDto {
            latest: LatestDto {
                release: v.latest.release.clone(),
                snapshot: v.latest.snapshot.clone(),
            },
            versions: v
                .versions
                .iter()
                .map(|v| VersionDto {
                    id: v.id.clone(),
                    version_type: v.version_type.clone(),
                })
                .collect(),
        }),
        Err(err) => {
            eprintln!("{}", err);
            None
        }
    }
}

pub async fn get_versions() -> Option<VersionsDto> {
    None
}
