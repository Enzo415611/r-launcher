#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct UserDto {
    pub username: String,
}

impl UserDto {
    pub fn new(username: String) -> Self {
        Self { username }
    }
}

impl ToString for UserDto {
    fn to_string(&self) -> String {
        self.username.to_string()
    }
}
