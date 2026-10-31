use iced::Task;

use crate::{
    db::{
        dto::user_dto::UserDto,
        service::user_service::{get_offline_users, remove_user, save_user},
    },
    events::events::Events,
    state::state::AppState,
    ui::view::{LoginPages, Page},
};

#[derive(Clone, Debug)]
pub enum LoginEvents {
    GetOffUsers,
    GetOffUsersFuture(Vec<UserDto>),
    CreateAccount,
    CreateAccountFuture(Option<UserDto>),
    RemoveAccount,
    RemoveAccountFuture(Result<(), toasty::Error>),
    DeviceCodeReceived { code: String, url: String },
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
                self.launcher_state.offline_accounts = v;
                Task::batch(vec![Task::done(Events::Config)])
            }
            LoginEvents::CreateAccount => {
                if self.ui_state.username.len() >= 3 {
                    if let Some(db) = self.db.to_owned() {
                        return Task::perform(
                            save_user(db, self.ui_state.username.to_string()),
                            |u| Events::Login(LoginEvents::CreateAccountFuture(u)),
                        );
                    }
                }
                Task::none()
            }
            LoginEvents::CreateAccountFuture(u) => {
                if let Some(u) = u {
                    self.launcher_state.current_account = Some(u.clone());
                    self.launcher_state.offline_accounts.push(u);
                    self.ui_state.username.clear();
                    self.ui_state.current_page = Page::Login(LoginPages::Home)
                }
                Task::none()
            }
            LoginEvents::RemoveAccount => {
                if let Some(current_account) = &self.launcher_state.current_account {
                    self.launcher_state
                        .offline_accounts
                        .retain(|u| u != current_account);
                    let id = current_account.id;
                    if let Some(u) = self.launcher_state.offline_accounts.first() {
                        self.launcher_state.current_account = Some(u.clone());
                        self.save_config();
                    } else {
                        self.launcher_state.current_account = None;
                    }

                    if let Some(db) = self.db.to_owned() {
                        return Task::perform(remove_user(db, id), |r| {
                            Events::Login(LoginEvents::RemoveAccountFuture(r))
                        });
                    }
                }
                Task::none()
            }
            LoginEvents::RemoveAccountFuture(r) => {
                if let Err(err) = r {
                    eprintln!("{}", err);
                }
                Task::none()
            }
            LoginEvents::DeviceCodeReceived { code, url } => Task::none(),
        }
    }
}
