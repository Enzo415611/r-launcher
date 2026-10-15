use crate::db::dto::versions_dto::VersionsDto;

const URL: &str = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

pub async fn get_versions() -> Option<VersionsDto> {
    match reqwest::get(URL).await {
        Ok(r) => match r.json::<VersionsDto>().await {
            Ok(versions) => Some(versions),
            Err(err) => {
                eprintln!("{}", err);
                None
            }
        },
        Err(err) => {
            eprintln!("{}", err);
            None
        }
    }
}
