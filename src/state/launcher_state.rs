use crate::{db::dto::user_dto::UserDto, launcher::versions_api::Versions};

pub struct LauncherState {
    pub current_account: Option<UserDto>,
    pub offline_accounts: Vec<UserDto>,
    pub versions: Option<Versions>,
}

impl LauncherState {
    pub fn new() -> Self {
        Self {
            current_account: None,
            offline_accounts: Vec::new(),
            versions: None,
        }
    }
}
