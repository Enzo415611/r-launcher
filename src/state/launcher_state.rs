use crate::{db::dto::user_dto::UserDto, launcher::versions_api::Versions};

pub struct LauncherState {
    pub current_account: Option<UserDto>,
    pub offline_accounts: Vec<UserDto>,
    pub version_state: VersionState,
}

pub struct VersionState {
    pub version_selected: Option<String>,
    pub versions_vec: Vec<String>,
    pub versions: Option<Versions>,
}

impl Default for VersionState {
    fn default() -> Self {
        Self {
            version_selected: None,
            versions: None,
            versions_vec: Vec::new(),
        }
    }
}

impl LauncherState {
    pub fn new() -> Self {
        Self {
            current_account: None,
            offline_accounts: Vec::new(),
            version_state: VersionState::default(),
        }
    }
}
