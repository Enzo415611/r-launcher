use toasty::Db;

use crate::db::{user_dto::UserDto, user_model::User};

pub async fn save_user(mut db: Db, username: String) -> Option<UserDto> {
    match User::create().username(username).exec(&mut db).await {
        Ok(n) => {
            return Some(UserDto {
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
                .map(|u| UserDto::new(u.username.to_string()))
                .collect();
            println!("aqui {:?}", v);
            return v;
        }
        Err(err) => {
            eprintln!("err: {}", err);
            return Vec::new();
        }
    }
}
