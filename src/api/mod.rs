use axum::{Router, routing::get};
use sqlx::{Pool, Postgres};
pub mod helloworld;
pub fn router(state: AppState) -> Router {
    let api_routes = Router::new()
        .route("/hello", get(helloworld::helloworld::helloworld));

    Router::new()
        .nest("/api", api_routes)
        .with_state(state)
}

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Pool<Postgres>
}
