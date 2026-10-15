#[derive(Debug, toasty::Model)]
pub struct Versions {
    #[key]
    pub id: String,
    pub latest: Latest,
    pub versions: Vec<Version>,
}

#[derive(Clone, Debug, Default, toasty::Embed)]
pub struct Latest {
    pub release: String,
    pub snapshot: String,
}

#[derive(Clone, Debug, Default, toasty::Embed)]
pub struct Version {
    pub id: String,
    pub version_type: String,
    pub url: Option<String>,
    pub time: Option<String>,
    pub release_time: Option<String>,
    pub sha1: Option<String>,
    pub compliance_level: i32,
}
