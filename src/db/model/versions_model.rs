#[derive(Debug, toasty::Model)]
pub struct Versions {
    #[key]
    pub id: String,
    pub latest: Latest,
    pub versions: Vec<Version>,
}

#[derive(Debug, toasty::Embed)]
pub struct Latest {
    pub release: String,
    pub snapshot: String,
}

#[derive(Debug, toasty::Embed)]
pub struct Version {
    pub id: String,
    pub version_type: String,
}
