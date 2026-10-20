use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]

pub struct VersionsDto {
    pub latest: LatestDto,
    pub versions: Vec<VersionDto>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Default, Debug)]
pub struct LatestDto {
    pub release: String,
    pub snapshot: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct VersionDto {
    pub id: String,
    #[serde(rename = "type")]
    pub version_type: String,
    pub url: Option<String>,
    pub time: Option<String>,
    #[serde(rename = "releaseTime")]
    pub release_time: Option<String>,
    pub sha1: Option<String>,
    #[serde(rename = "complianceLevel")]
    pub compliance_level: i32,
}
