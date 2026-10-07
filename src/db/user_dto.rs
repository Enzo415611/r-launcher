use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub struct UserDto {
    pub id: u64,
    pub username: String,
}

impl UserDto {
    pub fn new(id: u64, username: String) -> Self {
        Self { id, username }
    }
}

impl ToString for UserDto {
    fn to_string(&self) -> String {
        self.username.to_string()
    }
}
