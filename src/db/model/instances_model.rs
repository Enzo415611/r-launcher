#[derive(Debug, toasty::Model)]
pub struct InstanceModel {
    #[key]
    #[auto]
    pub id: u64,
    #[unique]
    pub name: String,
    pub loader_version: String,
    pub loader: String,
    pub version: String,
}

#[derive(Debug, toasty::Model)]
pub struct CurrentInstanceModel {
    #[key]
    pub id: String,
    pub name: String,
    pub loader_version: String,
    pub loader: String,
    pub version: String,
}
