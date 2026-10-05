use std::{fs, path::PathBuf};

use toasty::{Db, models};

use crate::db::user_model::User;

fn get_db_path() -> Option<PathBuf> {
    if let Some(path) = dirs::data_dir() {
        return Some(path.join("r-launcher"));
    }
    None
}

pub async fn init_db() -> Option<Db> {
    if let Some(p) = get_db_path() {
        fs::create_dir_all(&p).ok();
        let models = models!(User);
        if let Ok(db) = toasty::Db::builder()
            .models(models)
            .connect(&format!(
                "sqlite://{}/r-launcher.db",
                p.to_string_lossy().to_string()
            ))
            .await
        {
            if db.schema().db.tables.is_empty() {
                _ = db.push_schema().await;
            }
            return Some(db);
        }
    }
    None
}
