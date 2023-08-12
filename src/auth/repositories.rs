use std::marker::PhantomData;

use chrono::{DateTime, Utc};
use sqlx::{postgres::PgRow, Row};

use crate::models::user_models::{User, FilteredUser};

pub struct Repository<T> {
    data: PhantomData<T>
}

impl Repository<User> {
    pub fn from_row(row: &PgRow) -> Result<User, sqlx::Error> {
        let id: i32 = row.try_get("id")?;
        let name: String = row.try_get("name")?;
        let email: String = row.try_get("email")?;
        let password: String = row.try_get("password")?;
        let created_at: DateTime<Utc> = row.try_get("created_at")?;
        let updated_at: DateTime<Utc> = row.try_get("updated_at")?;
        
        Ok(User {
            id: id as u32,
            name,
            email,
            password,
            created_at,
            updated_at,
        })
    }
}