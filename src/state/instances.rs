#[derive(Default)]
pub struct InstanceState {
    pub instances: Vec<Instance>,
    pub is_create_mode: bool,
    pub create_instance: Instance,
}

#[derive(Debug, Clone)]
pub struct Instance {
    pub name: String,
    pub loader: Option<Loader>,
    // pub loader_versions: String,
    pub mc_version: Option<String>,
}

impl Default for Instance {
    fn default() -> Self {
        Self {
            name: String::new(),
            loader: Some(Loader::Vanilla),
            // loader_versions: String::new(),
            mc_version: None,
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

// impl From<lighty_launcher::Loader> for Loader {
//     fn from(value: lighty_launcher::Loader) -> Self {
//         match value {
//             lighty_launcher::Loader::LightyUpdater => Loader::LightyUpdater,
//             lighty_launcher::Loader::Fabric => Loader::Fabric,
//             lighty_launcher::Loader::Forge => Loader::Forge,
//             lighty_launcher::Loader::NeoForge => Loader::NeoForge,
//             lighty_launcher::Loader::Optifine => Loader::Optifine,
//             lighty_launcher::Loader::Quilt => Loader::Quilt,
//             lighty_launcher::Loader::Vanilla => Loader::Vanilla,
//         }
//     }
// }
