#[derive(Default)]
pub struct InstanceState {
    pub instances: Vec<Instance>,
    pub is_create_mode: bool,
    pub create_instance: Instance,
}

pub struct Instance {
    pub name: String,
    pub loader: lighty_launcher::Loader,
    pub versions: String,
}

impl Default for Instance {
    fn default() -> Self {
        Self {
            name: String::new(),
            loader: lighty_launcher::Loader::Vanilla,
            versions: String::new(),
        }
    }
}
