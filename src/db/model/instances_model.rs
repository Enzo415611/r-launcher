#[derive(Debug, toasty::Model)]
pub struct InstancesModel {
    #[key]
    id: u64,
    current_instance: InstanceModel,
    instances: Vec<InstanceModel>,
}
#[derive(Debug, toasty::Embed)]
pub struct InstanceModel {
    name: String,
    loader: String,
    version: String,
}
