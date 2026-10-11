use serde::{Deserialize, Serialize};

use crate::state::state::AppState;

const URL: &str = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

#[derive(Serialize, Deserialize, Clone)]
pub struct Versions {
    pub latest: Latest,
    pub versions: Vec<Version>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Latest {
    pub release: String,
    pub snapshot: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Version {
    pub id: String,
    #[serde(rename = "type")]
    pub version_type: String,
    pub url: String,
    pub time: String,
    #[serde(rename = "releaseTime")]
    pub release_time: String,
    pub sha1: String,
    #[serde(rename = "complianceLevel")]
    pub compliance_level: i32,
}

pub async fn get_versions() -> Option<Versions> {
    match reqwest::get(URL).await {
        Ok(r) => match r.json::<Versions>().await {
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
