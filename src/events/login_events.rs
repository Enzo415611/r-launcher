use iced::Task;

use crate::{
    db::{
        user_dto::UserDto,
        user_service::{get_offline_users, save_user},
    },
    events::events::Events,
    state::state::AppState,
    ui::view::{LoginPages, Page},
};

#[derive(Clone)]
pub enum LoginEvents {
    GetOffUsers,
    GetOffUsersFuture(Vec<UserDto>),
    CreateAccount,
    CreateAccountFuture(Option<UserDto>),
}

impl AppState {
    pub fn login_events(&mut self, e: LoginEvents) -> Task<Events> {
        match e {
            LoginEvents::GetOffUsers => {
                if let Some(db) = self.db.to_owned() {
                    return Task::perform(get_offline_users(db), |e| {
                        Events::Login(LoginEvents::GetOffUsersFuture(e))
                    });
                }
                Task::none()
            }
            LoginEvents::GetOffUsersFuture(v) => {
                self.launcher_state.offline_users = v;
                Task::none()
            }
            LoginEvents::CreateAccount => {
                if let Some(db) = self.db.to_owned() {
                    return Task::perform(save_user(db, self.ui_state.username.to_string()), |u| {
                        Events::Login(LoginEvents::CreateAccountFuture(u))
                    });
                }
                Task::none()
            }
            LoginEvents::CreateAccountFuture(u) => {
                if let Some(u) = u {
                    println!("{:?}", u);
                    self.launcher_state.current_account = Some(u.clone());
                    self.launcher_state.offline_users.push(u);
                    self.ui_state.username.clear();
                    self.ui_state.current_page = Page::Login(LoginPages::Home)
                }
                Task::none()
            }
        }
    }
}
