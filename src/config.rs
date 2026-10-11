use serde::{Deserialize, Serialize};

use crate::{db::dto::user_dto::UserDto, state::state::AppState};

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    current_account: Option<UserDto>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            current_account: None,
        }
    }
}

impl AppState {
    pub fn load_config(&mut self) {
        match confy::load::<AppConfig>("r-launcher", Some("r-launcher-config")) {
            Ok(cfg) => {
                if !self.launcher_state.offline_accounts.is_empty() {
                    self.launcher_state.current_account = cfg.current_account
                }
            }
            Err(err) => {
                eprintln!("{}", err);
            }
        }
    }

    pub fn save_config(&self) {
        let new_cfg = AppConfig {
            current_account: self.launcher_state.current_account.clone(),
        };
        match confy::store::<AppConfig>("r-launcher", Some("r-launcher-config"), new_cfg) {
            Ok(_) => {}
            Err(err) => {
                eprintln!("{}", err)
            }
        }
    }
}
