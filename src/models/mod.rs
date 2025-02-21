//! # Models module

use chrono::{DateTime, Utc};
use diesel::{Queryable, Selectable};

/// user
#[allow(dead_code, reason = "Work in progress")]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    /// Auto increment primary key
    pub id: i64,
    /// Unique identifier.
    pub username: String,
    /// Password
    pub hash: String,
    /// TOTP secret key
    pub totp: Option<String>,
    /// ID of the owner
    pub bot_owner_user_id: Option<i64>,
    /// Display name
    pub display_name: Option<String>,
    /// Bio.
    pub bio: Option<String>,
    /// Date when the user was created
    pub created_at: DateTime<Utc>,
    /// Date when the user was deleted
    pub deleted_at: Option<DateTime<Utc>>
}

/// key-value config storage
#[allow(dead_code, reason = "Work in progress")]
#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::configs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Config {
    /// Auto increment primary key
    pub id: i64,
    /// Config key (unique)
    pub key: String,
    /// Config value
    pub value: String
}
