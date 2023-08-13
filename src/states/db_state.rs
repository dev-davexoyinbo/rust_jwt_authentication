use sqlx::PgPool;

pub struct DBState {
    pub pool: PgPool,
}