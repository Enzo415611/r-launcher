#[derive(Default)]
pub struct InstanceState {
    pub instances: Vec<Instance>,
    pub is_create_mode: bool,
    pub create_instance: Instance,
}

#[derive(Debug, Clone)]
pub struct Instance {
    pub id: u64,
    pub name: String,
    pub loader: Option<Loader>,
    pub version: Option<String>,
}

impl Default for Instance {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
            loader: Some(Loader::Vanilla),
            // loader_versions: String::new(),
            version: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Loader {
    Fabric,
    Forge,
    NeoForge,
    Optifine,
    Quilt,
    Vanilla,
}

impl ToString for Loader {
    fn to_string(&self) -> String {
        match self {
            Self::Fabric => "Fabric".to_string(),
            Self::Forge => "Forge".to_string(),
            Self::NeoForge => "NeoForge".to_string(),
            Self::Optifine => "Optifine".to_string(),
            Self::Vanilla => "Vanilla".to_string(),
            Self::Quilt => "Quilt".to_string(),
        }
    }
}

pub fn to_loader(loader_name: &str) -> Loader {
    match loader_name {
        "Fabric" => Loader::Fabric,
        "Forge" => Loader::Forge,
        "NeoForge" => Loader::NeoForge,
        "Optifine" => Loader::Optifine,
        "Quilt" => Loader::Quilt,
        "Vanilla" => Loader::Vanilla,
        _ => Loader::Vanilla,
    }
}
