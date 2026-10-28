use indexmap::IndexSet;

use crate::state::instances;

#[derive(Default)]
pub struct InstanceState {
    pub instances: IndexSet<Instance>,
    pub is_create_mode: bool,
    pub is_edit_mode: bool,
    pub instance_selected: Instance,
    pub create_instance: Instance,
    pub instance_edit: Instance,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Instance {
    pub id: u64,
    pub name: String,
    pub loader: Option<Loader>,
    pub loader_version: String,
    pub mc_version: Option<String>,
}

impl Default for Instance {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
            loader: Some(Loader::Vanilla),
            loader_version: String::default(),
            mc_version: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

impl Loader {
    pub fn to_loader(name: &str) -> instances::Loader {
        match name {
            "Fabric" => Loader::Fabric,
            "Forge" => Loader::Forge,
            "NeoForge" => Loader::NeoForge,
            "Optifine" => Loader::Optifine,
            "Quilt" => Loader::Quilt,
            "Vanilla" => Loader::Vanilla,
            _ => Loader::Vanilla,
        }
    }
}

pub fn to_loader(loader_name: &str) -> lighty_launcher::Loader {
    match loader_name {
        "Fabric" => lighty_launcher::Loader::Fabric,
        "Forge" => lighty_launcher::Loader::Forge,
        "NeoForge" => lighty_launcher::Loader::NeoForge,
        "Optifine" => lighty_launcher::Loader::Optifine,
        "Quilt" => lighty_launcher::Loader::Quilt,
        "Vanilla" => lighty_launcher::Loader::Vanilla,
        _ => lighty_launcher::Loader::Vanilla,
    }
}
