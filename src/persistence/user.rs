use serde::{Deserialize, Serialize};
use sqlx::{Row, postgres::PgRow};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}

impl User {
    pub fn new(username: String, email: String) -> Self {
        Self {
            id: Uuid::now_v7(),
            username,
            email,
            bio: None,
            image: None,
        }
    }
}

pub struct UserPasswordHash(pub String);
