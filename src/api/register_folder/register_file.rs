use crate::api::register_folder::register_response::{RegisterResponse, UserData};
use crate::app_state::AppState;
use crate::persistence::user::{User, UserPasswordHash};
use crate::{api::register_folder::register_request::RegisterRequest, app_error::AppError};
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHasher};
use axum::{Json, extract::State};
use sqlx::{Postgres, Transaction, query};
use validator::Validate;

pub(crate) async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, AppError> {
    payload.validate().map_err(AppError::from_validation)?;

    let email: String = payload
        .user
        .email
        .parse()
        .map_err(|_| AppError::Internal("Invalid email format".into()))?;
    let user = payload.user;
    let mut tx: Transaction<'_, Postgres> = state
        .db_pool
        .begin()
        .await
        .map_err(|e| AppError::DatabaseError(sqlx::Error::BeginFailed))?;

    let registed_user = register_logic(&mut tx, user.username, email, user.password)
        .await
        .map_err(|e| {
            tracing::error!("DETAILED ERROR: {:?}", e);
            e
        })?;

    let user_response = UserData {
        email: registed_user.email,
        username: registed_user.username,
        bio: registed_user.bio,
        image: registed_user.image,
    };

    tx.commit()
        .await
        .map_err(|e| AppError::DatabaseError(sqlx::Error::WorkerCrashed))?;

    Ok(Json(RegisterResponse {
        user: user_response,
    }))
}

pub async fn register_logic(
    db: &mut Transaction<'_, Postgres>,
    username: String,
    email: String,
    password_raw: String,
) -> Result<User, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash_string = argon2
        .hash_password(password_raw.as_bytes(), &salt)
        .map_err(|e| AppError::Internal(e.to_string()))?
        .to_string();
    let hashed_password = UserPasswordHash(password_hash_string);
    let user = User::new(username, email);
    create(&mut *db, &user, &hashed_password).await?;

    Ok(user)
}

async fn create(
    tx: &mut Transaction<'_, Postgres>,
    user: &User,
    password_hash: &UserPasswordHash,
) -> Result<(), AppError> {
    query!(
        r#"
        INSERT INTO users (id, username, email, bio, image, password_hash)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        user.id,
        user.username,
        user.email,
        user.bio,
        user.image,
        password_hash.0
    )
    .execute(&mut **tx)
    .await
    .map_err(|e| {
        if let Some(db_err) = e.as_database_error()
            && db_err.code() == Some("23505".into())
        {
            let constraint = db_err.constraint().unwrap_or_default();

            if constraint.contains("email") {
                return AppError::Conflict {
                    field: "email".to_string(),
                    message: "has already been taken".to_string(),
                };
            } else if constraint.contains("username") {
                return AppError::Conflict {
                    field: "username".to_string(),
                    message: "has already been taken".to_string(),
                };
            }
        }
        AppError::DatabaseError(e)
    })?;
    Ok(())
}
