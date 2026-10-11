use serde::{Deserialize, Serialize};

use crate::state::state::AppState;

const URL: &str = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

#[derive(Serialize, Deserialize)]
struct Versions {
    latest: Latest,
    versions: Vec<Version>,
}

#[derive(Serialize, Deserialize)]
struct Latest {
    release: String,
    snapshot: String,
}

#[derive(Serialize, Deserialize)]
struct Version {
    id: String,
    #[serde(rename = "type")]
    versions_type: String,
    url: String,
    time: String,
    #[serde(rename = "releaseTime")]
    release_time: String,
    sha1: String,
    #[serde(rename = "complianceLevel")]
    compliance_level: i32,
}

impl AppState {
    pub fn get_versions(&self) {}
}
