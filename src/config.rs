use dotenvy::dotenv;
use serde::Deserialize;
use sqlx::ConnectOptions;
use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions};
use std::{str::FromStr, time::Duration};
use tracing::log::LevelFilter;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub server_port: u16,
    pub database_url: String,
    pub db_max_connections: u32,
    pub db_min_connections: u32,
    pub db_acquire_timeout_sec: u64,
    pub db_idle_timeout_sec: u64,

    pub jwt_secret: String,
    pub jwt_exp_hours: u64,

    pub rust_log: String,

    #[serde(default)]
    pub is_docker: bool,
}

impl Config {
    pub fn from_env() -> Result<Self, envy::Error> {
        dotenv().ok();

        envy::from_env::<Config>()
    }
}

pub async fn init_pool(config: &Config) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(config.db_max_connections)
        .min_connections(config.db_min_connections)
        .acquire_timeout(Duration::from_secs(config.db_acquire_timeout_sec))
        .idle_timeout(Duration::from_secs(config.db_idle_timeout_sec))
        .connect(&config.database_url)
        .await
}
