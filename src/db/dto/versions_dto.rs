#[derive(Clone)]
pub struct VersionsDto {
    pub latest: Option<LatestDto>,
    pub versions: Vec<VersionDto>,
}

#[derive(Clone, Default)]
pub struct LatestDto {
    pub release: String,
    pub snapshot: String,
}

#[derive(Clone)]
pub struct VersionDto {
    pub id: String,
    pub version_type: String,
    pub url: Option<String>,
    pub time: Option<String>,
    pub release_time: Option<String>,
    pub sha1: Option<String>,
    pub compliance_level: Option<i32>,
}
