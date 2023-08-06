use sqlx::types::chrono::{Utc, DateTime};

pub struct User {
    id: u32,
    name: String,
    email: String,
    password: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>
}

pub struct FilteredUser {
    id: u32,
    name: String,
    email: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>
}

