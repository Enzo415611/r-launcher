use iced::Task;
use toasty::Db;

use crate::{
    events::events::Events,
    state::{launcher_state::LauncherState, ui_state::UiState},
};

pub struct AppState {
    pub db: Option<Db>,
    pub current_theme: iced::Theme,
    pub ui_state: UiState,
    pub launcher_state: LauncherState,
}

impl AppState {
    pub fn new() -> (Self, Task<Events>) {
        (
            Self {
                db: None,
                current_theme: iced::Theme::Dark,
                ui_state: UiState::default(),
                launcher_state: LauncherState::new(),
            },
            Task::batch(vec![Task::done(Events::InitDb)]),
        )
    }
}
