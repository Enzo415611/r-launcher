use iced::Task;
use toasty::Db;

use crate::{
    db::db::init_db,
    events::{launcher_events::LauncherEvents, login_events::LoginEvents, ui_events::UiEvents},
    state::state::AppState,
};

#[derive(Clone)]
pub enum Events {
    InitDb,
    DbFuture(Option<Db>),
    Config,
    Ui(UiEvents),
    Login(LoginEvents),
    Launcher(LauncherEvents),
}

impl AppState {
    pub fn update(&mut self, event: Events) -> Task<Events> {
        match event {
            Events::InitDb => Task::perform(init_db(), Events::DbFuture),
            Events::DbFuture(db) => {
                self.db = db;
                Task::batch(vec![
                    Task::done(Events::Login(LoginEvents::GetOffUsers)),
                    Task::done(Events::Launcher(LauncherEvents::GetVersions)),
                ])
            }
            Events::Ui(e) => self.ui_events(e),
            Events::Login(e) => self.login_events(e),
            Events::Launcher(e) => self.launcher_events(e),
            Events::Config => {
                // load and save config
                self.load_config();
                Task::none()
            }
        }
    }
}
