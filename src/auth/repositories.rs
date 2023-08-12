use std::marker::PhantomData;

use chrono::{DateTime, Utc};
use sqlx::{postgres::PgRow, PgPool, Row};

use crate::models::user_models::{FilteredUser, User};

pub struct Repository<T> {
    data: PhantomData<T>,
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

    pub async fn get_by_email(pool: &PgPool, email: &str) -> Option<User> {
        let user = sqlx::query("SELECT * from users WHERE email = $1")
            .bind(&email)
            .map(|row: PgRow| Repository::<User>::from_row(&row).unwrap())
            .fetch_one(pool)
            .await;

        match user {
            Ok(user) => Some(user),
            Err(_) => None,
        }
    }

    pub async fn exist_by_email(pool: &PgPool, email: &str) -> bool {
        let exists: bool = sqlx::query("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)")
            .bind(email)
            .fetch_one(pool)
            .await
            .unwrap()
            .get(0);

        return exists;
    }

    pub async fn create_one(
        pool: &PgPool,
        email: &str,
        password: &str,
        name: &str,
    ) -> Result<Option<User>, sqlx::Error> {
        sqlx::query(
            "
            INSERT INTO 
                users (name, email, password, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5)
        ",
        )
        .bind(name)
        .bind(email)
        .bind(password)
        .bind(Utc::now())
        .bind(Utc::now())
        .execute(pool)
        .await?;

        let user = Self::get_by_email(pool, email).await;

        return Ok(user);
    }
}
