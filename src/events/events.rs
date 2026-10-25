use iced::Task;
use toasty::Db;

use crate::{
    db::db::init_db,
    events::{
        instance_events::InstanceEvents, launcher_events::LauncherEvents,
        login_events::LoginEvents, ui_events::UiEvents,
    },
    state::state::AppState,
};

#[derive(Debug, Clone)]
pub enum Events {
    InitDb,
    DbFuture(Option<Db>),
    Config,
    Ui(UiEvents),
    Login(LoginEvents),
    Launcher(LauncherEvents),
    Instance(InstanceEvents),
    ErrMsg(String),
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
                    Task::done(Events::Instance(InstanceEvents::GetAll)),
                ])
            }
            Events::Ui(e) => self.ui_events(e),
            Events::Login(e) => self.login_events(e),
            Events::Launcher(e) => self.launcher_events(e),
            Events::Config => {
                self.load_config();
                Task::none()
            }
            Events::Instance(e) => self.instance_events(e),
            Events::ErrMsg(err) => {
                eprintln!("{}", err);
                Task::none()
            }
        }
    }
}
