use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, serde::Serialize)]
pub struct FilteredUser {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TryFrom<User> for FilteredUser {
    type Error = &'static str;

    fn try_from(user: User) -> Result<Self, Self::Error> {
        Ok(FilteredUser { id: user.id, name: user.name, email: user.email, created_at: user.created_at, updated_at: user.updated_at })
    }
}