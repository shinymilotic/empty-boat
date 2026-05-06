use axum::{Json, extract::{State}};
use serde::Serialize;

use crate::{api::AppState, app_error::AppError};


pub(crate) async fn helloworld(
    State(state): State<AppState>,
) -> Result<Json<HellResponse>, AppError> {
    let hello: String = String::from("Hello, World!");

    Ok(Json(HellResponse { hello }))
}

#[derive(Debug, Serialize)]
pub struct HellResponse {
    pub hello: String,
}

