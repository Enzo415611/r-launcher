use iced::Task;
use toasty::Db;

use crate::{db::db::init_db, events::ui_events::UiEvents, state::state::AppState};

#[derive(Clone)]
pub enum Events {
    InitDb,
    DbInitialized(Option<Db>),
    UiEvents(UiEvents),
}

impl AppState {
    pub fn update(&mut self, event: Events) -> Task<Events> {
        match event {
            Events::InitDb => Task::perform(init_db(), Events::DbInitialized),
            Events::DbInitialized(db) => {
                self.db = db;
                Task::none()
            }
            Events::UiEvents(e) => self.ui_events(e),
        }
    }
}
