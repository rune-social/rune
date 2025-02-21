//! # Database Module

use deadpool_diesel::{
    Runtime,
    postgres::{BuildError, Manager, Pool}
};

/// Initialize the database connection.
pub fn init(database_url: &str) -> Result<Pool, BuildError> {
    let manager = Manager::new(database_url, Runtime::Tokio1);
    Pool::builder(manager).build()
}
