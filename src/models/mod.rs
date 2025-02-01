//! # Models module

use chrono::{DateTime, Utc};
use diesel::{Queryable, Selectable};

/// user
#[allow(dead_code, reason = "Work in progress")]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    /// id of user. pk
    pub id: i64,
    /// username. unique
    pub username: String,
    /// password hash of user
    pub hash: String,
    /// totp secret key
    pub totp: Option<String>,
    /// owner user id if this user is bot
    pub bot_owner_user_id: Option<i64>,
    /// display name
    pub display_name: Option<String>,
    /// bio
    pub bio: Option<String>,
    /// created_at
    pub created_at: DateTime<Utc>,
    /// deleted_at. null if not deleted
    pub deleted_at: Option<DateTime<Utc>>
}

/// key-value config storage
#[allow(dead_code, reason = "Work in progress")]
#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::configs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Config {
    /// id for config
    pub id: i64,
    /// key for config (unique)
    pub key: String,
    /// value for config
    pub value: String
}
