use serde::{Deserialize, Deserializer};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(nested)]
    pub user: RegisterUser,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterUser {
    #[validate(length(min = 1, message = "can't be blank"))]
    pub username: String,

    #[validate(
        length(min = 1, message = "can't be blank"),
        email(message = "is invalid")
    )]
    pub email: String,

    #[validate(length(min = 1, message = "can't be blank"))]
    pub password: String,
}

fn deserialize_resettable<'de, D, T>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    Ok(Some(Option::<T>::deserialize(deserializer)?))
}
