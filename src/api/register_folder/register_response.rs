use serde::{Deserialize, Serialize};

use crate::persistence::user::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterResponse {
    pub user: UserData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub email: String,
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
}

impl UserData {
    pub(crate) fn new(user: User) -> Self {
        UserData {
            email: user.email,
            username: user.username,
            bio: user.bio,
            image: user.image,
        }
    }
}
