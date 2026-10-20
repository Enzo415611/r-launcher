use iced::Task;
use toasty::Db;

use crate::{
    events::events::Events,
    state::{instances::InstanceState, launcher_state::LauncherState, ui_state::UiState},
};

pub struct AppState {
    pub db: Option<Db>,
    pub ui_state: UiState,
    pub launcher_state: LauncherState,
    pub instances_state: InstanceState,
}

impl AppState {
    pub fn new() -> (Self, Task<Events>) {
        (
            Self {
                db: None,
                ui_state: UiState::default(),
                launcher_state: LauncherState::new(),
                instances_state: InstanceState::default(),
            },
            Task::batch(vec![Task::done(Events::InitDb)]),
        )
    }
}
