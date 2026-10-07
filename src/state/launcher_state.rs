use crate::db::user_dto::UserDto;

pub struct LauncherState {
    pub current_account: Option<UserDto>,
    pub offline_accounts: Vec<UserDto>,
}

impl LauncherState {
    pub fn new() -> Self {
        Self {
            current_account: None,
            offline_accounts: Vec::new(),
        }
    }
}
