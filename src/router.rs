use axum::{Router, routing::post};

use crate::{api::register_folder::register_file, app_state::AppState};

pub fn router(state: AppState) -> Router {
    let api_routes = Router::new().route("/register", post(register_file::register));

    Router::new().nest("/api", api_routes).with_state(state)
}
