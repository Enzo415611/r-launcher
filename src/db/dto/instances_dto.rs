#[derive(Debug, Clone)]
pub struct InstancesDto {
    current_instance: InstanceDto,
    instances: Vec<InstanceDto>,
}

#[derive(Debug, Clone)]
pub struct InstanceDto {
    name: String,
    loader: String,
    versions: String,
}
