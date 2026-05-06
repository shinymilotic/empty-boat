
pub mod config;
pub mod api;
pub mod app_error;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;
use crate::{api::{AppState, router}, config::{from_env, init_pool}};

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = from_env().expect("Failed to load configuration");

    let db_pool = init_pool(&config).await?;

    let app_state: AppState = AppState { db_pool  };
    let app = router(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));
    let listener = TcpListener::bind(addr).await?;

    info!("Starting server on {}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}
