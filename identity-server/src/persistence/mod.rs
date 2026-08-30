use std::sync::LazyLock;

use sqlx::{Pool, Postgres, pool::PoolConnection};

use crate::config::CONFIG;

static DB_POOL: LazyLock<Pool<Postgres>> = LazyLock::new(|| {
    let database_url = CONFIG.database_url();
    Pool::<Postgres>::connect_lazy(database_url).expect("Failed to create database pool")
});

pub enum DbError {
    ConnectionError(sqlx::Error),
}

async fn get_connection() -> Result<PoolConnection<Postgres>, DbError> {
    DB_POOL.acquire().await.map_err(DbError::ConnectionError)
}
