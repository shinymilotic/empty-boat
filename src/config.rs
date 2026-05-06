use dotenvy::dotenv;
use serde::Deserialize;
use sqlx::ConnectOptions;
use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions};
use std::{time::Duration};
use tracing::log::LevelFilter;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub server_port: u16,
    pub db_host: String,
    pub db_port: u16,
    pub db_username: String,
    pub db_password: String,
    pub db_name: String,
    pub db_max_connections: u32,
    pub db_min_connections: u32,
    pub db_acquire_timeout_sec: u64,
    pub db_idle_timeout_sec: u64,
}

pub fn from_env() -> Result<Config, envy::Error> {
    dotenv().ok();

    envy::from_env::<Config>()
}


pub async fn init_pool(config: &Config) -> Result<PgPool, sqlx::Error> {

    PgPoolOptions::new()
        .max_connections(config.db_max_connections)
        .min_connections(config.db_min_connections)
        .acquire_timeout(Duration::from_secs(config.db_acquire_timeout_sec))
        .idle_timeout(Duration::from_secs(config.db_idle_timeout_sec))
        .connect_with(
            PgConnectOptions::new()
                .host(&config.db_host)
                .port(config.db_port)
                .username(&config.db_username)
                .password(&config.db_password)
                .database(&config.db_name)
                .log_statements(LevelFilter::Debug)
                .log_slow_statements(LevelFilter::Warn, Duration::from_millis(100)),
        )
        .await
}

