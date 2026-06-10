pub mod api;
pub mod app_error;
pub mod app_state;
pub mod config;
pub mod persistence;
pub mod router;
use crate::{app_state::AppState, config::init_pool, router::router};
use envy::from_env;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let config = from_env().expect("Failed to load configuration");

    let db_pool = init_pool(&config).await?;

    let app_state: AppState = AppState { db_pool };
    let app = router(app_state);
    print!("{0}", config.server_port);
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));
    let listener = TcpListener::bind(addr).await?;

    info!("Starting server on {}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}
