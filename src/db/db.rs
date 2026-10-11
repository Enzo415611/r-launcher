use std::{fs, path::PathBuf};

use toasty::{Db, models};

use crate::db::model::{user_model::User, versions_model::Versions};

fn get_db_path() -> Option<PathBuf> {
    if let Some(path) = dirs::data_dir() {
        return Some(path.join("r-launcher"));
    }
    None
}

pub async fn init_db() -> Option<Db> {
    if let Some(p) = get_db_path() {
        fs::create_dir_all(&p).ok();
        let models = models!(User, Versions);
        let db_path = p.join("r-launcher.db");
        let is_new_db = !db_path.exists();
        if let Ok(db) = toasty::Db::builder()
            .models(models)
            .connect(&format!(
                "sqlite://{}",
                db_path.to_string_lossy().to_string()
            ))
            .await
        {
            if is_new_db {
                if let Err(e) = db.push_schema().await {
                    eprintln!("{}", e);
                    return None;
                }
            }

            return Some(db);
        }
    }
    None
}
