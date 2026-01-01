use crate::config::AppConfig;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

pub async fn init_db(config: AppConfig) -> PgPool {
    PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(10))
        .connect(&config.pg_url)
        .await
        .expect("failed to connect to Postgres")
}
