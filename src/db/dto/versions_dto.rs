pub struct VersionsDto {
    pub latest: LatestDto,
    pub versions: Vec<VersionDto>,
}

pub struct LatestDto {
    pub release: String,
    pub snapshot: String,
}

pub struct VersionDto {
    pub id: String,
    pub version_type: String,
}
