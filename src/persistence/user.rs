use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}

impl User {
    pub fn new(username: String, email: String) -> Self {
        Self {
            id: 0,
            username,
            email,
            bio: None,
            image: None,
        }
    }
}

pub struct UserPasswordHash(pub String);
