use chrono::{DateTime, Utc};


#[derive(Debug)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

#[derive(Debug)]
pub struct FilteredUser {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

