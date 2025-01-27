//! # Database Module

use std::str::FromStr;

use sqlx::{
    Error,
    MySqlPool,
    mysql::{MySqlConnectOptions, MySqlPoolOptions, MySqlSslMode}
};
use tracing::log::LevelFilter;

/// Initialize the database connection.
pub async fn init(database_url: &str) -> Result<MySqlPool, Error> {
    // pipes_as_concat: we won't use this, and PlanetScale's MySQL distribution
    // breaks with this enabled
    let conn_opts = MySqlConnectOptions::from_str(database_url)?
        .ssl_mode(MySqlSslMode::Required)
        .charset("utf8mb4")
        .pipes_as_concat(false)
        .timezone(String::from("+00:00"));

    MySqlPoolOptions::new()
        .max_connections(4000)
        .min_connections(num_cpus::get() as u32)
        .acquire_slow_level(LevelFilter::Warn)
        .after_connect(|_conn, _meta| Box::pin(async move { Ok(()) }))
        .connect_with(conn_opts)
        .await
}
