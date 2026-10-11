use toasty::Db;

use crate::db::{dto::user_dto::UserDto, model::user_model::User};

pub async fn save_user(mut db: Db, username: String) -> Option<UserDto> {
    match User::create().username(username).exec(&mut db).await {
        Ok(n) => {
            return Some(UserDto {
                id: n.id,
                username: n.username,
            });
        }
        Err(err) => {
            eprintln!("{}", err);
            return None;
        }
    }
}

pub async fn get_offline_users(mut db: Db) -> Vec<UserDto> {
    match User::all().exec(&mut db).await {
        Ok(u) => {
            let v: Vec<UserDto> = u
                .into_iter()
                .map(|u| UserDto::new(u.id, u.username.to_string()))
                .collect();
            return v;
        }
        Err(err) => {
            eprintln!("err: {}", err);
            return Vec::new();
        }
    }
}

pub async fn remove_user(mut db: Db, id: u64) -> Result<(), toasty::Error> {
    match User::delete_by_id(&mut db, id).await {
        Ok(_) => Ok(()),
        Err(err) => {
            eprintln!("err: {}", err);
            Err(err.into())
        }
    }
}
