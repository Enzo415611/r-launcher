#[derive(Debug, Clone, Default)]
pub struct InstanceDto {
    pub id: u64,
    pub name: String,
    pub loader: String,
    pub loader_version: String,
    pub version: String,
}
