use crate::{
    db::{user_dto::UserDto, user_model::User},
    state::state::AppState,
};

impl AppState {
    pub async fn save_user(&mut self, username: String) {
        if let Some(mut db) = self.db.to_owned() {
            if let Ok(new_user) = User::create().username(username).exec(&mut db).await {
                self.launcher_state.offline_users.push(UserDto {
                    username: new_user.username,
                });
            };
        }
    }

    pub async fn get_offline_users(&self) -> Vec<UserDto> {
        if let Some(mut db) = self.db.to_owned() {
            if let Ok(u) = User::all().exec(&mut db).await {
                let v: Vec<UserDto> = u
                    .iter()
                    .map(|u| UserDto::new(u.username.to_string()))
                    .collect();
                return v;
            }
        }
        Vec::new()
    }
}
